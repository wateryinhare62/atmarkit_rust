use std::path::Path;

fn main() {
    let file_str = "src/bin/path_is.rs";
    let path_file = Path::new(file_str);
    println!("Of {} ...", file_str);
    println!("Canonical: {}", path_file.canonicalize().unwrap().to_str().unwrap());
    println!("File: {}", path_file.file_name().unwrap().to_str().unwrap());
    println!("Stem: {}", path_file.file_stem().unwrap().to_str().unwrap());
    println!("Extenstion: {}", path_file.extension().unwrap().to_str().unwrap());
}