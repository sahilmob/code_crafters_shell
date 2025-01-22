use std::process::Command;

pub static TYPE: &str = "pwd";

pub fn pwd() -> String {
    format!(
        "{}",
        String::from_utf8(Command::new("pwd").output().unwrap().stdout).unwrap()
    )
}
