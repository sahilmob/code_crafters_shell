pub fn split_path(path: &str) -> Vec<String> {
    path.split(":").map(String::from).collect()
}
