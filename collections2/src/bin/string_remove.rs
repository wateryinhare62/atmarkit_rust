//Cargoをnightly版にしないとコメントを除去できません。
//#![feature(string_remove_matches)]
fn main() {
    let mut english = String::from("Hello, world!");
    let mut japanese = String::from("こんにちは、もうひとつの世界！");
    let mut c = english.remove(2);
    println!("{}", c);
    c = japanese.remove(18);
    println!("{}", c);
    //english.remove_matches("o");
    //println!("{}", english);
}
