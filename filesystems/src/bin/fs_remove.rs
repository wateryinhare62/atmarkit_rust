use std::fs;

fn main() {
    let d = "temp";
    let f = "temp/hello.txt";
    fs::remove_dir_all(d).unwrap();
    fs::create_dir(d).unwrap();
    fs::write(f, "Hello").unwrap();
    fs::remove_file(f).unwrap();
    fs::remove_dir(d).unwrap();
}