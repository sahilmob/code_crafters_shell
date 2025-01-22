mod commands;

use commands::*;
use std::io::{self, Write};

fn run(cmds: &mut Vec<String>) -> String {
    if cmds.is_empty() {
        return "".to_string();
    }

    let cmd = cmds.remove(0);
    let args = cmds;
    let mut cmd_args: Vec<String> = Vec::new();

    while !args.is_empty() {
        if args[0] == "|" {
            break;
        }

        cmd_args.push(args.remove(0));
    }

    let result = match cmd {
        cmd if cmd == "exit" => exit::exit(&mut cmd_args),
        cmd if cmd == "echo" => echo::echo(&mut cmd_args),
        cmd if cmd == "type" => r#type::r#type(&mut cmd_args),
        cmd if !cmd.is_empty() => format!("{}: command not found", cmd),
        _ => "".to_string(),
    };

    if !args.is_empty() {
        args.push(result);

        return run(args);
    }

    result
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let cmd_iter = input.trim().split(" ");
        let mut cmds: Vec<String> = cmd_iter.map(|s| s.to_string()).collect();

        if !cmds.is_empty() {
            println!("{}", run(&mut cmds));
        }
    }
}
