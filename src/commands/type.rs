use crate::{
    echo, exit,
    lib::helpers::{check_exec_path::*, split_path::*},
};
use std::env;

pub static TYPE: &str = "type";

#[macro_export]
macro_rules! typ {
    ($($key: path),*) => {
        {
            use ::std::collections::HashMap;
            let mut hm: HashMap<&str, String> = HashMap::new();

            $ (
                let mut s = String::from($key.to_string());
                s.push_str(" is a shell builtin");
                hm.insert($key, s);
            )*

            hm
       }
    };
}

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

    let hm = typ!(exit::TYPE, echo::TYPE, self::TYPE);

    let cmd = args.remove(0);

    match hm.get(cmd.as_str()) {
        Some(v) => format!("{}", v),
        None => match env::var("PATH") {
            Ok(v) => format!("{}", handle_executables(v, &cmd)),
            Err(_) => format!("{}: not found", cmd),
        },
    }
}
