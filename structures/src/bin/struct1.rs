struct Person {
    name: String,
    birth: u32,
    sex: char,
    height: f64,
    weight: f64,
}

fn main() {
    let nao = Person {
        birth: 1945,
        height: 160.0,
        name: String::from("山内直"),
        sex: 'm',
        weight: 80.0,
    };
}
