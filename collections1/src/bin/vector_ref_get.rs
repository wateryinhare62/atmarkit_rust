fn main() {
    let mut months = Vec::new();
    months.push("January");
    months.push("February");
    months.push("March");
    months.push("April");
    println!("今月は {}", months.get(2).unwrap());
}
