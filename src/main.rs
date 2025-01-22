mod commands;
mod internal;

use commands::*;
use exec_bin::exec_bin;
use internal::helpers::drain_current_cmd_args::*;
use std::io::{self, Write};

fn run(cmds: &mut Vec<String>) -> String {
    if cmds.is_empty() {
        return "".to_string();
    }

    let cmd = cmds.remove(0);
    let args = cmds;
    let mut cmd_args = drain_current_cmd_args(args);

    let result = match cmd {
        cmd if cmd == "exit" => exit::exit(&mut cmd_args),
        cmd if cmd == "echo" => echo::echo(&mut cmd_args),
        cmd if cmd == "type" => r#type::r#type(&mut cmd_args),
        _ => {
            args.insert(0, cmd);
            exec_bin(args)
        } // cmd if !cmd.is_empty() => format!("{}: command not found", cmd),
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

        let mut cmds: Vec<String> = input.trim().split(" ").map(|s| s.to_string()).collect();

        if !cmds.is_empty() {
            println!("{}", run(&mut cmds));
        }
    }
}
