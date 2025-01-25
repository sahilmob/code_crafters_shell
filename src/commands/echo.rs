pub static TYPE: &str = "echo";

pub fn echo(args: &mut Vec<String>) -> Result<String, String> {
    if args.is_empty() {
        return Ok("".to_string());
    }

    Ok(format!("{}", args.join("").trim()))
}
