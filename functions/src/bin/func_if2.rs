fn main() {
    let a = divider(5, 2);
}

fn divider(x: i32, y: i32) -> i32 {
    if y != 0 {
        return x / y;
    } else {
        return 0;
    }
}
