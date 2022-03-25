fn main() {
    let mut months: Vec<&String> = Vec::new();
    let jan = &String::from("January");
    months.push(jan);
    let feb = &String::from("February");
    months.push(feb);
    println!("Jan: {}", months[0]);
    println!("Count: {}, {:?}", months.len(), months);
}
