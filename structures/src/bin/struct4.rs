struct Person {
    name: String,
    birth: u32,
    sex: char,
    height: f64,
    weight: f64,
}

fn main() {
    let someone = Person {
        name: "名無しの権兵衛".to_string(),
        birth: 1945,
        sex: 'm',
        height: 160.0,
        weight: 80.0,
    };
    
    let nao = Person {
        name: "山内直".to_string(),
        birth: 1960,
        ..someone
    };
}
