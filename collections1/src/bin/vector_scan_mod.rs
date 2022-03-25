fn main() {
    let mut months = Vec::new();
    months.push(String::from("January"));
    months.push(String::from("February"));
    months.push(String::from("March"));
    months.push(String::from("April"));
    for month in &mut months {
        *month = month.to_uppercase();
    }
    println!("{}", months[0]);
}
