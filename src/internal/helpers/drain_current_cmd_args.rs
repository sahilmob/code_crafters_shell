pub fn drain_current_cmd_args(args: &mut Vec<String>) -> Vec<String> {
    let mut cmd_args: Vec<String> = Vec::new();

    while !args.is_empty() && args[0] != "|" {
        cmd_args.push(args.remove(0));
    }

    cmd_args
}
