use std::path::Path;

fn main() {
    let path_str = Path::new("str.txt");
    let string = String::from("string.txt");
    let path_string = Path::new(&string);
    let path_path = Path::new(path_string);
    println!("&str: {}", path_str.to_str().unwrap());
    println!("String: {}", path_string.to_str().unwrap());
    println!("Path: {}", path_path.to_str().unwrap());
}