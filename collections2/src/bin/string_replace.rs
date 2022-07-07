fn main() {
    let english = String::from("Hello, world!");
    let japanese = String::from("こんにちは、世界！");
    let mut res = english.replace('o', "a");
    println!("{}", res);
    res = japanese.replace("世界", "World");
    println!("{}", res);
}
