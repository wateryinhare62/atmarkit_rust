use std::path::Path;

fn main() {
    let file_str = "src/bin/path_iter.rs";
    let path_file = Path::new(file_str);
    println!("Part of {}", file_str);
    for elem in path_file.iter() {
        println!("{}", elem.to_str().unwrap());
    }
}