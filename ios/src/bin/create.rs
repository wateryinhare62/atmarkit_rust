use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

fn main() -> Result<()> {
    let filename = "wagahaiwa_inudearu.txt";
    let mut file = File::create(filename)?;
    file.write_all(b"I am a dog.\n")?;
    Ok(())
}
