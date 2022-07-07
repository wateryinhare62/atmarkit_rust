fn main() {
    let english = String::from("Hello, world!");
    let japanese = String::from("こんにちは、世界！");
    for c in english.chars() {
        print!("{} ", c);
    }
    println!();
    for c in japanese.char_indices() {
        print!("{}:{} ", c.0, c.1);
    }
    println!();
}
