fn main() {
    let x = 100;
    let y = function_copy(x);
    println!("xは{}、yは{}です。", x, y);
}

fn function_copy(a: i32) -> i32 {
    println!("function_copy: 引数aの値は{}です。", a);
    a
}
