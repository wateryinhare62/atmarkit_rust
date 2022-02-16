use std::env;

fn main() 
{
    for arg in env::args() {
        println!("引数は {} です。", arg);
    }
}