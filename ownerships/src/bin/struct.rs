struct Date {
    year: i32,
    month: i32,
    day: i32,
}


fn main() {
    let d1 = Date {year:2021, month:10, day:14};
    let d2 = d1;
    println!("d1は{}です。", d1.year);
    println!("d2は{}です。", d2.year);
}
