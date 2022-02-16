enum Creature {
    ANIMAL,
    BIRD,
    INSECT,
    FISH,
}

fn main() {
    let lion = Creature::ANIMAL;

    match lion {
        Creature::ANIMAL => println!("ライオンはANIMALです。"),
        Creature::BIRD => println!("ライオンはBIRDです。"),
        Creature::INSECT => println!("ライオンはINSECTです。"),
        Creature::FISH => println!("ライオンはFISHです。"),
        _ => println!("どれでもないみたいです。"),
    }
}
