use std::{
    env,
    io::{self, Write},
    path::{Path, PathBuf},
};

use nix::unistd::{AccessFlags, access};

const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdout().flush()?;
        stdin.read_line(&mut input)?;
        let command: Vec<&str> = input.split_whitespace().collect(); // need to handle quotes
        // should check for error here
        let result = match command[0] {
            "exit" => return Ok(()),
            "echo" => {
                echo(&command[1..]);
                Ok(())
            }
            "type" => {
                type_command(&command[1..]);
                Ok(())
            }
            "pwd" => pwd(),
            "cd" => cd(&command[1..]),
            other => {
                if let Some(path) = find_executable(other) {
                    execute(&path, &command[1..])?;
                    Ok(())
                } else {
                    Err(format!("{}: command not found", other).into())
                }
            }
        };
        if let Err(e) = result {
            println!("{}", e);
        }
        io::stdout().flush()?;
        input.clear();
    }
}

fn echo(args: &[&str]) {
    for arg in args {
        print!("{} ", arg);
    }
    println!();
}

fn type_command(args: &[&str]) {
    if args.len() != 1 {
        return;
    }
    let arg = args[0];
    if BUILTINS.contains(&arg) {
        println!("{} is a shell builtin", arg);
    } else if let Some(path) = find_executable(arg) {
        println!("{} is {}", arg, path.display());
    } else {
        println!("{}: not found", arg);
    }
}

fn find_executable(name: &str) -> Option<PathBuf> {
    let path = std::env::var("PATH").unwrap_or_default();
    for dir in path.split(':') {
        let mut path = PathBuf::from(dir);
        path.push(name);
        if path.is_file() && is_executable(&path) {
            return Some(path);
        }
    }
    None
}

fn is_executable(path: &PathBuf) -> bool {
    access(path, AccessFlags::X_OK).is_ok()
}

fn execute(path: &Path, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new(path.file_name().unwrap_or_default())
        .args(args)
        .status()?;
    Ok(())
}

fn pwd() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}

fn cd(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() > 1 {
        return Err("cd: too many arguments".into());
    }
    let path = if args.is_empty() || args[0] == "~" {
        let home = env::var("HOME").unwrap_or_default();
        PathBuf::from(home)
    } else {
        PathBuf::from(args[0])
    };
    if !path.exists() {
        return Err(format!("cd: {}: No such file or directory", path.display()).into());
    }
    env::set_current_dir(&path)?;
    Ok(())
}
