use std::fs;

fn main() {
    fs::rename("neko.txt", "cat.txt").unwrap();
    fs::rename("cat.txt", "temp/cat.txt").unwrap();
}