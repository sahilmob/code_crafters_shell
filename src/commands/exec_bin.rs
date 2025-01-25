use std::process::Command;

use crate::{
    drain_current_cmd_args,
    internal::{
        constants::bin_paths::BIN_PATHS,
        helpers::{
            check_exec_path::*, remove_empty_spaces_from_args::remove_empty_spaces_from_args,
        },
        types::cmd_output::CmdOutput,
    },
};

pub static TYPE: &str = "exec_bin";

pub fn handle_executables(cmd: &str, args: &mut Vec<String>) -> CmdOutput {
    for p in BIN_PATHS.iter() {
        let local_path = format!("{}/{}", p, cmd);
        if check_exec_path(&local_path) {
            let args = drain_current_cmd_args(args);
            let output = Command::new(cmd).args(args).output().unwrap();

            let result = match String::from_utf8(output.stdout) {
                Ok(v) => Some(v.trim().to_string()),
                Err(_) => None,
            };

            let err = match String::from_utf8(output.stderr) {
                Ok(e) => {
                    if !e.trim().is_empty() {
                        Some(e.trim().to_string())
                    } else {
                        None
                    }
                }
                Err(e) => Some(e.to_string()),
            };

            return (result, err);
        }
    }

    (None, Some(format!("{}: command not found", cmd)))
}

pub fn exec_bin(args: &mut Vec<String>) -> CmdOutput {
    if args.is_empty() {
        return (None, Some("not found".to_string()));
    }

    let mut args = remove_empty_spaces_from_args(args);
    let cmd = args.remove(0);

    match BIN_PATHS.len() {
        n if n > 0 => handle_executables(&cmd, &mut args),
        _ => (Some(format!("{}: command not found", cmd)), None),
    }
}
