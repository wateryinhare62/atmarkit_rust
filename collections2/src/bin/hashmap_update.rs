use std::collections::HashMap;

fn main() {
    let mut months = HashMap::new();
    months.insert("January", 1);
    months.insert("February", 2);
    months.insert("March", 3);
    months.insert("April", 3);
    months.insert("April", 4);
    println!("Aprilは {}月です。", months.get("April").unwrap());
    months.entry("May").or_insert(5);
    println!("Mayは {}月です。", months.get("May").unwrap());
}
