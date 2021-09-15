fn main() {
    let age = 15;

    if age >= 25 {
        println!("選挙権と被選挙権があります。");
    } else if age >= 18 {
        println!("選挙権のみがあります。");
    } else {
        println!("選挙権も被選挙権もありません。");
    }
}
