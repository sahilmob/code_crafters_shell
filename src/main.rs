mod commands;
mod internal;

use cd::cd;
use commands::*;
use exec_bin::exec_bin;
use internal::{cmd_parser::cmd_parser, helpers::drain_current_cmd_args::*};
use std::io::{self, Write};

fn run(cmds: &mut Vec<String>) -> String {
    let cmd = cmds.remove(0);
    let args = cmds;
    let mut cmd_args = drain_current_cmd_args(args);

    let result = match cmd {
        cmd if cmd == pwd::TYPE => pwd::pwd(),
        cmd if cmd == cd::TYPE => cd::cd(&mut cmd_args),
        cmd if cmd == exit::TYPE => exit::exit(&mut cmd_args),
        cmd if cmd == echo::TYPE => echo::echo(&mut cmd_args),
        cmd if cmd == r#type::TYPE => r#type::r#type(&mut cmd_args),
        _ => {
            cmd_args.insert(0, cmd);
            exec_bin(&mut cmd_args)
        }
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

        if input.trim().is_empty() {
            continue;
        }

        let mut cmds: Vec<String> = cmd_parser::parse(input);

        if !cmds.is_empty() {
            let result = run(&mut cmds);
            if !result.is_empty() {
                println!("{}", result);
            }
        }
    }
}
