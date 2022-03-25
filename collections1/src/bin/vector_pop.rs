fn main() {
    let mut months = Vec::new();
    months.push("January");
    months.push("February");
    months.push("March");
    months.push("April");
    println!("取り出したのは {} で要素数は {}", months.pop().unwrap(), months.len());
}
