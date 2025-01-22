use std::env;
use std::{collections::HashSet, sync::LazyLock};

pub static BIN_PATHS: &LazyLock<HashSet<&str>> = &LazyLock::new(|| match env::var("PATH") {
    Ok(v) => v.split(":").collect::<HashSet<_>>(),
    Err(_) => HashSet::new(),
});
