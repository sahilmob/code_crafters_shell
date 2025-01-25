use std::io::Write;
use std::process::{exit, Command, Stdio};

use crate::{
    drain_current_cmd_args,
    internal::{
        constants::bin_paths::BIN_PATHS,
        helpers::{
            check_exec_path::*, remove_empty_spaces_from_args::remove_empty_spaces_from_args,
        },
    },
};

pub static TYPE: &str = "exec_bin";

pub fn handle_path_cmd(cmd: &str, args: &[&String], mut handle: Box<dyn Write>) {
    let output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    match output {
        Ok(output) => {
            handle.write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
        }
        Err(e) => {
            eprintln!("Failed to execute {}: {}", cmd, e);
        }
    }
}

pub fn handle_executables(cmd: &str, args: &mut Vec<String>) -> String {
    for p in BIN_PATHS.iter() {
        let local_path = format!("{}/{}", p, cmd);
        if check_exec_path(&local_path) {
            let args = drain_current_cmd_args(args);

            return match Command::new(cmd).args(args).output() {
                Ok(v) => match String::from_utf8(v.stdout) {
                    Ok(v) => v.trim().to_string(),
                    Err(e) => e.to_string(),
                },
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

    let mut args = remove_empty_spaces_from_args(args);
    let cmd = args.remove(0);

    match BIN_PATHS.len() {
        n if n > 0 => format!("{}", handle_executables(&cmd, &mut args)),
        _ => format!("{}: command not found", cmd),
    }
}
