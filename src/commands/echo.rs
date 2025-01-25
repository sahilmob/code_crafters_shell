use crate::internal::types::cmd_output::CmdOutput;

pub static TYPE: &str = "echo";

pub fn echo(args: &mut Vec<String>) -> CmdOutput {
    if args.is_empty() {
        return (Some("".to_string()), None);
    }

    (Some(format!("{}", args.join("").trim())), None)
}
