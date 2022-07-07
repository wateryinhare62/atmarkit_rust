fn main() {
    let mut months = String::new();
    months.push('J');
    months.push('a');
    months.push('n');
    months.push('u');
    months.push('a');
    months.push('r');
    months.push('y');
    months.push_str(",February");
    months.push_str(&String::from(",March"));
    months.push_str(&String::from(",April"));
    println!("{}", months);
}
