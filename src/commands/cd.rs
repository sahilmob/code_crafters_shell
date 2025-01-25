use std::env;
use std::path::Path;

use crate::internal::helpers::remove_empty_spaces_from_args::remove_empty_spaces_from_args;
use crate::internal::types::cmd_output::CmdOutput;

pub static TYPE: &str = "cd";

pub fn cd(args: &mut Vec<String>) -> CmdOutput {
    if args.is_empty() {
        return (Some("".to_string()), None);
    }

    let arg = remove_empty_spaces_from_args(args).remove(0);
    let path = if arg == "~" {
        match env::var("HOME") {
            Ok(home_path) => Path::new(&home_path).to_path_buf(),
            Err(_) => {
                println!("HOME not found");
                Path::new(".").to_path_buf()
            }
        }
    } else {
        Path::new(&arg).to_path_buf()
    };

    match env::set_current_dir(path) {
        Ok(_) => (Some("".to_string()), None),
        Err(_) => (
            None,
            Some(format!("cd: {}: No such file or directory", arg)),
        ),
    }
}
