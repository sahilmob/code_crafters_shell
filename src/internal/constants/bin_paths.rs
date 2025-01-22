use std::env;
use std::{collections::HashSet, sync::LazyLock};

pub static BIN_PATHS: LazyLock<HashSet<String>> = LazyLock::new(|| match env::var("PATH") {
    Ok(v) => v.split(":").map(String::from).collect::<HashSet<String>>(),
    Err(_) => HashSet::new(),
});
