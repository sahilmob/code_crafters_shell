use crate::{
    cmd_types::*,
    internal::helpers::{check_exec_path::*, split_path::*},
};
use std::env;

pub static TYPE: &str = "type";

pub fn handle_executables(path: String, cmd: &str) -> String {
    let exec_paths = split_path(&path);

    for p in exec_paths {
        let local_path = format!("{}/{}", p, cmd);
        if check_exec_path(&local_path) {
            return format!("{} is {}", cmd, local_path);
        }
    }

    format!("{} not found", cmd)
}

pub fn r#type(args: &mut Vec<String>) -> String {
    if args.is_empty() {
        return "not found".to_string();
    }

    let hs = &CMD_TYPES;
    let cmd = args.remove(0);

    match hs.get(cmd.as_str()) {
        Some(v) => format!("{} is a shell builtin", v),
        None => match env::var("PATH") {
            Ok(v) => format!("{}", handle_executables(v, &cmd)),
            Err(_) => format!("{}: not found", cmd),
        },
    }
}
