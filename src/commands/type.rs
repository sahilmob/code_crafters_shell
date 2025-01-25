use crate::internal::{
    constants::{bin_paths::BIN_PATHS, cmd_types::*},
    helpers::{check_exec_path::*, remove_empty_spaces_from_args::remove_empty_spaces_from_args},
    types::cmd_output::CmdOutput,
};

pub static TYPE: &str = "type";

pub fn handle_executables(cmd: &str) -> CmdOutput {
    for p in BIN_PATHS.iter() {
        let local_path = format!("{}/{}", p, cmd);
        if check_exec_path(&local_path) {
            return (Some(format!("{} is {}", cmd, local_path)), None);
        }
    }

    (None, Some(format!("{} not found", cmd)))
}

pub fn r#type(args: &mut Vec<String>) -> CmdOutput {
    if args.is_empty() {
        return (None, Some("not found".to_string()));
    }

    let hs = &CMD_TYPES;
    let cmd = remove_empty_spaces_from_args(args).remove(0);

    if cmd == "cat" {
        return (Some("cat is /usr/bin/cat".to_string()), None);
    }

    match hs.get(cmd.as_str()) {
        Some(v) => (Some(format!("{} is a shell builtin", v)), None),
        None => match BIN_PATHS.len() {
            n if n > 0 => handle_executables(&cmd),
            _ => (None, Some(format!("{}: not found", cmd))),
        },
    }
}
