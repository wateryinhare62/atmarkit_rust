fn main() {
    let english = String::from("Hello, world!");
    let japanese = String::from("こんにちは、世界！");
    let world = &english[7..12];
    //let sekai = &japanese[6..8];
    let sekai = &japanese[18..24];
    println!("{} は {}", world, sekai);
}
