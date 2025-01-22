pub static TYPE: &str = "echo";

pub fn echo(args: &mut Vec<&str>) {
    println!("{}", args.join(" "));
}
