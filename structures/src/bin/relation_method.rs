struct Person {
    name: String,
    birth: u32,
    sex: char,
    height: f64,
    weight: f64,
}

impl Person {
    fn from_str(name: &str, birth: u32, sex: char) -> Person {
        Person {
            name: name.to_string(),
            birth,
            sex,
            height: 0.0,
            weight: 0.0,
        }
    }
}

fn main() {
    let nao = Person::from_str("山内直", 1945, 'm');
}
