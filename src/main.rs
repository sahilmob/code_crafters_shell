mod commands;

use commands::*;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut cmd_iter = input.trim().split(" ");
        let cmd = cmd_iter.next();
        let args: Vec<&str> = cmd_iter.collect();

        match cmd {
            Some(v) => match v {
                v if v == "exit" => {
                    exit::exit(args);
                }
                v if v == "echo" => {
                    echo::echo(args);
                }
                v if v == "type" => {
                    r#type::typ(args);
                }
                n if !v.is_empty() => {
                    println!("{}: command not found", n);
                }
                _ => (),
            },
            None => (),
        }
    }
}
