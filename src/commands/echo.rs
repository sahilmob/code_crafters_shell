pub static TYPE: &str = "echo";

pub fn echo(args: &mut Vec<String>) -> String {
    if args.is_empty() {
        return "".to_string();
    }

    format!("{}", args.join(""))
}
