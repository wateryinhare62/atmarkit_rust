fn main() {
    let a = multiplier(5, 10);
    println!("aは{}です。", a);
}

fn multiplier(x: i32, mut y: i32) -> i32 {
    let mut r = 0;
    while y > 0 {
        r += x;
        y -= 1;
    }
    r
}
