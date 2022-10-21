use std::fs::OpenOptions;
use std::io::Result;
use std::io::prelude::*;

fn main() -> Result<()> {
    let filename = "wagahaiwa_inudearu.txt";
    let mut options = OpenOptions::new();
    let mut file = options.append(true).open(filename)?;
    file.write_all(b"I have no name, yet.\n")?;
    Ok(())
}
