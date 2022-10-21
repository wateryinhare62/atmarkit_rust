use std::path::Path;

fn main() {
    let file_str = "src/bin/path_is.rs";
    let dir_str = "target";
    let path_file = Path::new(file_str);
    let path_dir = Path::new(dir_str);
    println!("{} is file: {}", file_str, path_file.is_file());
    println!("{} is directory: {}", dir_str, path_dir.is_dir());
    println!("{} is absolute: {}", dir_str, path_dir.is_absolute());
    println!("{} is relative: {}", dir_str, path_dir.is_relative());
    println!("{} has root: {}", dir_str, path_dir.has_root());
    println!("{} exists: {}", file_str, path_file.exists());
    println!("{} starts with 'src': {}", file_str, path_file.starts_with("src"));
    println!("{} ends with 'path_is.rs': {}", 
        file_str, path_file.ends_with("path_is.rs"));
}