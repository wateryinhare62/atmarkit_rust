fn main() {
    let s = String::from("Hello, Rust!!");
    let t = &s[0..5];
    println!("スライスは「{}」です。", t);
}
