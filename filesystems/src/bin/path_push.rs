use std::path::{Path, PathBuf};

fn main() {
    let mut path_buf = PathBuf::new();
    let path_file = Path::new("src/bin/path_push.rs");
    path_buf.push(path_file);
    println!("PathBuf: {}", path_buf.as_path().to_str().unwrap());
    path_buf.clear();
    path_buf.push("src");
    path_buf.push("bin");
    path_buf.push("path_push.rs");
    println!("PathBuf: {}", path_buf.as_path().to_str().unwrap());
}