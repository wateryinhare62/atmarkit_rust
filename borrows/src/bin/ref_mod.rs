fn main() {
    let mut s = String::from("Hello");

    println!("変更前の文字列は「{}」です。", s);
    change_string(&mut s);
    println!("変更された文字列は「{}」です。", s);
}

fn change_string(s: &mut String) {
    s.push_str(", Rust!!");
}
