fn main() {
    let s = dangling_function();
    println!("sã¯{}ã§ãã", s);
}

fn dangling_function() -> &String {
    let s = String::from("hello");
    &s
}
