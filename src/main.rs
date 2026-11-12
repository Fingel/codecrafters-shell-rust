#[allow(unused_imports)]
use std::io::{self, Write};

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
