// 注意：このファイルはコンパイルエラーになります。
fn main() {
    let mut s = String::from("Hello, Rust!!");
    let t = &s[0..5];
    s.clear();
    println!("スライスは「{}」です。", t);
}
