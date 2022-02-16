use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    match read_file(filename) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("エラーが起きています：{}", e),
    };
}

fn read_file(filename: &String) -> Result<String, io::Error>
{
    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => Ok(contents),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}
