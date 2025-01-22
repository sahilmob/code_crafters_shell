use std::env::{self, *};

pub static TYPE: &str = "pwd";

pub fn pwd() -> String {
    env::current_dir().unwrap().display().to_string()
}
