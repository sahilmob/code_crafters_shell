pub fn remove_empty_spaces_from_args(args: &mut Vec<String>) -> Vec<String> {
    args.iter()
        .filter(|a| !a.trim().is_empty())
        .map(|s| s.clone())
        .collect::<Vec<String>>()
}
