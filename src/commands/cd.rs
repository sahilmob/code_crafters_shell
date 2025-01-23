use std::env;
use std::path::Path;

pub static TYPE: &str = "cd";

pub fn cd(args: &mut Vec<String>) -> String {
    if args.is_empty() {
        return "".to_string();
    }

    let arg = args.remove(0);
    let path = Path::new(&arg);

    if path.is_absolute() {
        match env::set_current_dir(path) {
            Ok(_) => "".trim().to_string(),
            Err(_) => format!("cd: {}: No such file or directory", arg),
        }
    } else {
        "".to_string()
    }
}
