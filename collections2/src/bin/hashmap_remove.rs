use std::collections::HashMap;

fn main() {
    let mut months = HashMap::new();
    months.insert("January", 1);
    months.insert("February", 2);
    months.insert("March", 3);
    months.insert("April", 4);
    let march = months.remove("March");
    if march != None {
        println!("Marchは削除されて {}月でした。", march.unwrap());
    }
}
