use std::io::{self, Result, Write};

fn main() -> Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        stdout.write(b"> ")?;
        stdout.flush()?;
        match stdin.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                stdout.write_all(buffer.as_bytes())?;
                buffer.clear();
            }
            Err(_e) => break,
        }
    }
    Ok(())
}