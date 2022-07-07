fn main() {
    let mut english = String::from("Helo, world!");
    let mut japanese = String::from("こんにちは、世界！");
    english.insert(2, 'l');
    println!("{}", english);
    japanese.insert_str(18, "もうひとつの");
    println!("{}", japanese);
}
