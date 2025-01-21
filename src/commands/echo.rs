pub static TYPE: &str = "echo";

pub fn echo(args: Vec<&str>) {
    println!("{}", args.join(" "));
}
