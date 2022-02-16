use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            println!("{}", contents);
        },
        Err(error) => {
            println!("ファイル {} が見つかりませんのでやり直して下さい。", filename);
            println!("エラーは {} です。", error);
        },
    };
}
