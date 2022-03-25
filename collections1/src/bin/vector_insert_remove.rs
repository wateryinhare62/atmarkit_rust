fn main() {
    let mut months = Vec::new();
    months.push("January");
    months.push("March");
    months.push("April");
    months.insert(1, "February");
    println!("Count: {}, {:?}", months.len(), months);
    months.remove(0);
    println!("Count: {}, {:?}", months.len(), months);
}
