use std::fs;

fn main() {
    let s = "ningen_shikkaku.txt";
    fs::write(s, "by O. Dazai").unwrap();
    println!("read: {}", String::from_utf8(fs::read(s).unwrap()).unwrap());
    println!("read_to_string: {}", fs::read_to_string(s).unwrap());
}