use std::env;
use std::path::Path;

pub static TYPE: &str = "cd";

pub fn cd(args: &mut Vec<String>) -> String {
    if args.is_empty() {
        return "".to_string();
    }

    let arg = args
        .iter()
        .filter(|a| !a.trim().is_empty())
        .collect::<Vec<_>>()
        .remove(0);
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
        Ok(_) => "".to_string(),
        Err(_) => format!("cd: {}: No such file or directory", arg),
    }
}
