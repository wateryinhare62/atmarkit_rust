fn main() {
    let mut s = String::from("Hello, world!");
    s = function_move(s);
    println!("sは「{}」です。", s);
}

fn function_move(m: String) -> String {
    println!("function_move: 引数mの値は「{}」です。", m);
    m
}