use crate::internal::{
    constants::{bin_paths::BIN_PATHS, cmd_types::*},
    helpers::{check_exec_path::*, remove_empty_spaces_from_args::remove_empty_spaces_from_args},
};

pub static TYPE: &str = "type";

pub fn handle_executables(cmd: &str) -> Result<String, String> {
    for p in BIN_PATHS.iter() {
        let local_path = format!("{}/{}", p, cmd);
        if check_exec_path(&local_path) {
            return Ok(format!("{} is {}", cmd, local_path));
        }
    }

    Err(format!("{} not found", cmd))
}

pub fn r#type(args: &mut Vec<String>) -> Result<String, String> {
    if args.is_empty() {
        return Err("not found".to_string());
    }

    let hs = &CMD_TYPES;
    let cmd = remove_empty_spaces_from_args(args).remove(0);

    match hs.get(cmd.as_str()) {
        Some(v) => Ok(format!("{} is a shell builtin", v)),
        None => match BIN_PATHS.len() {
            n if n > 0 => handle_executables(&cmd),
            _ => Ok(format!("{}: not found", cmd)),
        },
    }
}
