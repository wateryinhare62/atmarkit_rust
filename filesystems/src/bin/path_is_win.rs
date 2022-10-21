use std::path::Path;

fn main() {
    let path_file = Path::new("src\\bin\\path_is.rs");
    let path_dir = Path::new("target");
    println!("{} is file: {}", path_file.to_str().unwrap(), path_file.is_file());
    println!("{} is directory: {}", path_dir.to_str().unwrap(), path_dir.is_dir());
    println!("{} is absolute: {}", path_dir.to_str().unwrap(), path_dir.is_absolute());
    println!("{} is relative: {}", path_dir.to_str().unwrap(), path_dir.is_relative());
    println!("{} has root: {}", path_dir.to_str().unwrap(), path_dir.has_root());
    println!("{} is exists: {}", path_file.to_str().unwrap(), path_file.exists());
    println!("{} is starts with 'src': {}", path_file.to_str().unwrap(), path_file.starts_with("src"));
    println!("{} is ends with 'path_is.rs': {}", path_file.to_str().unwrap(), path_file.ends_with("path_is.rs"));
}