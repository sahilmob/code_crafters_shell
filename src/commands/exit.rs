use crate::internal::helpers::remove_empty_spaces_from_args::remove_empty_spaces_from_args;

pub static TYPE: &str = "exit";

pub fn exit(args: &mut Vec<String>) -> Result<String, String> {
    if args.is_empty() {
        std::process::exit(0)
    }

    let code = remove_empty_spaces_from_args(args).remove(0);

    Ok(std::process::exit(code.parse::<i32>().unwrap_or(0)))
}
