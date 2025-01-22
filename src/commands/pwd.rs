use std::process::Command;

pub static TYPE: &str = "pwd";

pub fn pwd() -> String {
    match Command::new("pwd").output() {
        Ok(v) => match String::from_utf8(v.stdout) {
            Ok(v) => v.trim().to_string(),
            Err(e) => e.to_string(),
        },
        Err(e) => e.to_string(),
    }
}
