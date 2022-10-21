use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

fn main() -> Result<()> {
    let filename = "wagahaiwa_nekodearu.txt";
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
