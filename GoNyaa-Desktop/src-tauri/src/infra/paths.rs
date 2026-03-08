use std::collections::HashSet;
use std::path::PathBuf;

pub fn legacy_search_dirs() -> Vec<PathBuf> {
    let mut ordered = Vec::new();
    let mut seen = HashSet::new();

    let mut push = |path: PathBuf| {
        if seen.insert(path.clone()) {
            ordered.push(path);
        }
    };

    if let Ok(current_dir) = std::env::current_dir() {
        push(current_dir.clone());
        push(current_dir.join("GoNyaa-Server"));
        if let Some(parent) = current_dir.parent() {
            push(parent.to_path_buf());
            push(parent.join("GoNyaa-Server"));
        }
    }

    if let Ok(executable) = std::env::current_exe() {
        if let Some(dir) = executable.parent() {
            push(dir.to_path_buf());
            push(dir.join("GoNyaa-Server"));
            if let Some(parent) = dir.parent() {
                push(parent.to_path_buf());
                push(parent.join("GoNyaa-Server"));
            }
        }
    }

    ordered
}
