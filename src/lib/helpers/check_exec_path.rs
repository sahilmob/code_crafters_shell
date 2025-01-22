use std::path::Path;

pub fn check_exec_path(p: &str) -> bool {
    let path = Path::new(&p);
    if path.exists() {
        return true;
    }

    false
}
