fn main() {
    let mut months = Vec::new();
    let jan = String::from("January");
    months.push(jan);
    //months.push("February");
    //months.push("March");
    //months.push("April");
    println!("Jan: {}", jan);
    println!("Count: {}, {:?}", months.len(), months);
}
