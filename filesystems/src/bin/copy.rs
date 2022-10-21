use std::fs;

fn main() {
    fs::copy("wagahaiwa_nekodearu.txt", "neko.txt").unwrap();
}