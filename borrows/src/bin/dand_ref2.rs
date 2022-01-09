fn main() {
    let s = dangling_function();
    println!("sは{}です。", s);
}

fn dangling_function() -> &String {
    let s = String::from("hello");
    &s
}
