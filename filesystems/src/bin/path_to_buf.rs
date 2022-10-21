use std::path::{Path, PathBuf};

fn main() {
    let path_file = Path::new("src/bin/path_is.rs");
    let path_buf = path_file.to_path_buf();
    let path_buf_file = path_file.with_file_name("source.c");
    let path_buf_ext = path_file.with_extension("bak");
    println!("PathBuf: {}", path_buf.as_path().to_str().unwrap());
    println!("PathBuf with filename: {}", path_buf_file.as_path().to_str().unwrap());
    println!("PathBuf with extension: {}", path_buf_ext.as_path().to_str().unwrap());
}