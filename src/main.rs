#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdout().flush()?;
        stdin.read_line(&mut input)?;
        println!("{}: command not found", input.trim());
        io::stdout().flush()?;
        input.clear();
    }
}
