use std::collections::HashMap;

fn main() {
    let keys = vec!["January", "February", "March", "April"];
    let values = vec![1, 2, 3, 4];
    let months: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    println!("{:?}", months);
}
