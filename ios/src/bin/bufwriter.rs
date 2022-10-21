use std::fs::File;
use std::io::{BufRead, BufReader, Write, BufWriter};

fn main() -> Result<(), Box<std::error::Error>> {
    let sf = "wagahaiwa_nekodearu.txt";
    let df = "cat.txt";
    let mut bw = BufWriter::new(File::create(df)?);
    for res in BufReader::new(File::open(sf)?).lines() {
        let line = res?;
        writeln!(bw, "{}", line);
    }
    Ok(())
}