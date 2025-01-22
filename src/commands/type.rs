use crate::{echo, exit};
use std::env;
use std::path::Path;

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

fn split_path(path: &str) -> Vec<String> {
    path.split(":").map(String::from).collect()
}

fn find_exec_in_dir(p: &str) -> bool {
    let path = Path::new(&p);
    if path.exists() {
        return true;
    }

    false
}

pub fn handle_executables(path: String, cmd: &str) -> String {
    let exec_paths = split_path(&path);

    for p in exec_paths {
        let local_path = format!("{}/{}", p, cmd);
        if find_exec_in_dir(&local_path) {
            return format!("{} is {}", cmd, local_path);
        }
    }

    format!("{} not found", cmd)
}

pub fn r#type(args: &mut Vec<&str>) {
    if args.is_empty() {
        println!("not found")
    }

    let hm = typ!(exit::TYPE, echo::TYPE, self::TYPE);

    let cmd = &args.pop().unwrap();

    match hm.get(cmd) {
        Some(v) => println!("{}", v),
        None => match env::var("PATH") {
            Ok(v) => {
                println!("{}", handle_executables(v, &cmd))
            }
            Err(_) => println!("{}: not found", cmd),
        },
    };
}
