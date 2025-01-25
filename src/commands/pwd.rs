use std::env;

pub static TYPE: &str = "pwd";

pub fn pwd() -> Result<String, String> {
    Ok(env::current_dir().unwrap().display().to_string())
}
