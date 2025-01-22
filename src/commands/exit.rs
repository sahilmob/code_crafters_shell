pub static TYPE: &str = "exit";

pub fn exit(args: &mut Vec<String>) -> String {
    if args.is_empty() {
        std::process::exit(0)
    }

    let code = args.remove(0);

    std::process::exit(code.parse::<i32>().unwrap_or(0))
}
