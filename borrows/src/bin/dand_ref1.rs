fn main() {
    let s: &String;
    let c = 'w';

    let pos = search_position(&s, c);
    println!("文字'{}'の「{}」中の位置は{}文字目です。", c, s, pos);
}

fn search_position(s: &String, c: char) -> usize {
    let pos = s.find(c).unwrap();
    pos
}
