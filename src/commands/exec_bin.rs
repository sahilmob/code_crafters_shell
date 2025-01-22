use crate::{
    drain_current_cmd_args,
    internal::{constants::bin_paths::BIN_PATHS, helpers::check_exec_path::*},
};
use std::process::Command;

pub static TYPE: &str = "exec_bin";

pub fn handle_executables(cmd: &str, args: &mut Vec<String>) -> String {
    for p in BIN_PATHS.iter() {
        let local_path = format!("{}/{}", p, cmd);
        if check_exec_path(&local_path) {
            let args = drain_current_cmd_args(args);
            let mut command = Command::new(cmd);
            args.iter().for_each(|a| {
                command.arg(a);
            });

            return match command.output() {
                Ok(v) => String::from_utf8(v.stdout).unwrap(),
                Err(e) => e.to_string(),
            };
        }
    }

    format!("{}: command not found", cmd)
}

pub fn exec_bin(args: &mut Vec<String>) -> String {
    if args.is_empty() {
        return "not found".to_string();
    }

    let cmd = args.remove(0);

    match BIN_PATHS.len() {
        n if n > 0 => format!("{}", handle_executables(&cmd, args)),
        _ => format!("{}: not found", cmd),
    }
}
