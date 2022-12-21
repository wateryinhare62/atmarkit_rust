macro_rules! printargs {
    () => {
        print!("No argument.\n");
    };
    ($($arg:expr),*) => {
        {
            $(println!("Argument: {}", $arg);)*
        }
    };
}
  
fn main() {
    printargs!("One", "Two", "Three");
    printargs!["Four", "Five"];
    printargs!{};
}
