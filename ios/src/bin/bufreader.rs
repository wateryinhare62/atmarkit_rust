use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<std::error::Error>> {
    let f = "wagahaiwa_nekodearu.txt";
    let mut line_no = 1;
    for res in BufReader::new(File::open(f)?).lines() {
        let line = res?;
        println!("{}: {}", line_no, line);
        line_no = line_no + 1;
    }
    Ok(())
}