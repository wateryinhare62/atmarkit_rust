fn main() {
    let s = String::from("Hello, world!");
    function_move(s);
    println!("sは「{}」です。", s);
}

fn function_move(m: String) {
    println!("function_move: 引数mの値は「{}」です。", m);
}
