#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdout().flush()?;
        stdin.read_line(&mut input)?;
        let command = input.trim();
        match command {
            "exit" => return Ok(()),
            _ => println!("{}: command not found", command),
        }
        io::stdout().flush()?;
        input.clear();
    }
}
