fn main() {
    let mut months = Vec::new();
    months.push("January");
    months.push("February");
    months.push("March");
    months.push("April");
    for month in &months {
        print!("{} ", month);
    }
    println!();
}
