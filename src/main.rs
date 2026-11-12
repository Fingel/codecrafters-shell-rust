use std::{
    io::{self, Write},
    path::PathBuf,
};

use nix::unistd::{AccessFlags, access};

const BUILTINS: &[&str] = &["exit", "echo", "type"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdout().flush()?;
        stdin.read_line(&mut input)?;
        let command: Vec<&str> = input.split_whitespace().collect(); // need to handle quotes
        match command[0] {
            "exit" => return Ok(()),
            "echo" => echo(&command[1..]),
            "type" => type_command(&command[1..]),
            _ => println!("{}: command not found", command[0]),
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
