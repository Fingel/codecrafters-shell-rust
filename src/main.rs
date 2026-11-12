use std::io::{self, Write};

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
    if BUILTINS.contains(&args[0]) {
        println!("{} is a shell builtin", args[0]);
    } else {
        println!("{}: not found", args[0]);
    }
}
