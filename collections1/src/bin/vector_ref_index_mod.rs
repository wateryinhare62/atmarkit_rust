fn main() {
    let mut months = Vec::new();
    months.push("January");
    months.push("February");
    months.push("March");
    months.push("April");
    let month = &months[1]; 
    months.push("May");
    println!("今月は {}", month);
}
