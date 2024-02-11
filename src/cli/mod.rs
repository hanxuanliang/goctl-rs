use std::path::PathBuf;

pub mod oli;

mod error;

fn canonicalize_path(path: PathBuf) -> PathBuf {
    path.canonicalize().unwrap_or_else(|e| {
        let path_str = path.to_str().unwrap_or("Invalid path");
        eprintln!("Error resolving path '{}': {}", path_str, e);
        std::process::exit(1);
    })
}
