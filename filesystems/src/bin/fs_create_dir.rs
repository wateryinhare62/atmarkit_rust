use std::fs;

fn main() {
    fs::create_dir("temp").unwrap();
    fs::create_dir_all("temp/one/two/three").unwrap();
}