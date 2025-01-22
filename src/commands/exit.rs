pub static TYPE: &str = "exit";

pub fn exit(args: &mut Vec<&str>) {
    if args.is_empty() {
        std::process::exit(0)
    }

    std::process::exit(args[0].parse::<i32>().unwrap_or(0));
}
