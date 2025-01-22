mod commands;

use commands::*;
use std::io::{self, Write};

fn do_work(cmds: Vec<&str>) {
    let mut cmds_iter = cmds.iter();
    let cmd = cmds_iter.next();
    let mut args = cmds_iter.map(|s| *s).collect::<Vec<&str>>();

    match cmd {
        Some(cmd) => match cmd {
            cmd if *cmd == "exit" => {
                exit::exit(&mut args);
            }
            cmd if *cmd == "echo" => {
                echo::echo(&mut args);
            }
            cmd if *cmd == "type" => {
                r#type::r#type(&mut args);
            }
            cmd if !cmd.is_empty() => {
                println!("{}: command not found", cmd);
            }
            _ => (),
        },
        None => (),
    }
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let cmd_iter = input.trim().split(" ");
        let cmds: Vec<&str> = cmd_iter.collect();

        if !cmds.is_empty() {
            do_work(cmds);
        }
    }
}
