fn main() {
    let a = 5;
    let b = 10;
    println!("{}の二乗は{}、{}の二乗は{}です。", a, square1(a), b, square2(b));
}

fn square1(x: i32) -> i64 {
    return x as i64 * x as i64;
}

fn square2(x: i32) -> i64 {
    x as i64 * x as i64
    //(x * x).into()
}
