use std::env;

use crate::internal::types::cmd_output::CmdOutput;

pub static TYPE: &str = "pwd";

pub fn pwd() -> CmdOutput {
    (
        Some(env::current_dir().unwrap().display().to_string()),
        None,
    )
}
