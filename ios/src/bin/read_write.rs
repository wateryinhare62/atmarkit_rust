use std::fs::OpenOptions;
use std::io::SeekFrom;
use std::io::prelude::*;

fn main() -> Result<(), Box<std::io::Error>> {
    let filename = "wagahaiwa_inudearu.txt";
    let s = b"I am a dog.\n";
    let mut options = OpenOptions::new();
    let mut file = options.read(true).write(true).create(true).open(filename)?;
    file.write(s)?;
    file.seek(SeekFrom::Start(0))?;
    let mut buf: [u8; 10] = Default::default();
    file.read(&mut buf)?;
    println!("{:?}", buf);
    Ok(())
}
