struct Person {
    name: String,
    birth: u32,
    sex: char,
    height: f64,
    weight: f64,
}

impl Person {
    fn bmi(&self) -> f64 {
      self.weight /  ((self.height / 100.0) * (self.height / 100.0))
    }
}

fn main() {
    let nao = Person {
        name: "山内直".to_string()),
        birth: 1945,
        sex: 'm',
        height: 160.0,
        weight: 80.0,
    };

    println!("{}さんのBMIは{}です。", nao.name, nao.bmi());
}
