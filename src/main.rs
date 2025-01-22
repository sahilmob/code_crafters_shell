mod commands;

use commands::*;
use std::io::{self, Write};

fn work_loop(cmd: &str, args: &mut Vec<&str>) {
    match cmd {
        cmd if cmd == "exit" => {
            exit::exit(args);
        }
        cmd if cmd == "echo" => {
            echo::echo(args);
        }
        cmd if cmd == "type" => {
            r#type::typ(args);
        }
        cmd if !cmd.is_empty() => {
            println!("{}: command not found", cmd);
        }
        _ => (),
    }
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut cmd_iter = input.trim().split(" ");
        let cmd = cmd_iter.next();
        let mut args: Vec<&str> = cmd_iter.collect();

        match cmd {
            Some(cmd) => {
                work_loop(cmd, &mut args);
            }
            None => (),
        }
    }
}
