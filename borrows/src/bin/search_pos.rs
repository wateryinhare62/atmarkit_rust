fn main() {
    let s1 = String::from("Hello, world!!");
    let c = 'w';

    let (s2, pos) = search_position(s1, c);
    println!("文字'{}'の「{}」中の位置は{}文字目です。", c, s2, pos);
}

fn search_position(s: String, c: char) -> (String, usize) {
    let pos = s.find(c).unwrap();
    (s, pos)
}
