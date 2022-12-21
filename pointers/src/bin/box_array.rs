const ARRAY_SIZE: usize = 10000;

fn main() {
    let mut a: Box<[i32; ARRAY_SIZE]> = Box::new([0; ARRAY_SIZE]);
    let mut i = 1;
    for elem in a.iter_mut() {
        *elem = i;
        i = i + 1;
    }
    for elem in a.iter() {
        print!("{} ", elem);
    }
    println!();
}