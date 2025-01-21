#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if input.trim() == "exit 0" {
            std::process::exit(0)
        }

        println!("{}: command not found", input.trim());
    }
}
