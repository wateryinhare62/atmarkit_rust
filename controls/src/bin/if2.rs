fn main() {
    let age = 15;

    let s = if age >= 25 {
        "選挙権と被選挙権があります。"
    } else if age >= 18 {
        "選挙権のみがあります。"
    } else {
        "選挙権も被選挙権もありません。"
    };
    println!("{}", s);
}
