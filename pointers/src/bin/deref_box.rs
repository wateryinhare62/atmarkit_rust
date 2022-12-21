fn main() {
    let a = Box::new(100);
    let r = &a;
    println!("a = {}, r = {}", a, *r);
    println!("a = {}, r = {}", a, r);
}