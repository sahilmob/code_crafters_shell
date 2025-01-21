mod commands;

use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            n if n == "exit 0" => {
                std::process::exit(0);
            }
            n if n.starts_with("echo") => {
                commands::echo::echo(n);
            }
            n => {
                println!("{}: command not found", n);
            }
        }
    }
}
