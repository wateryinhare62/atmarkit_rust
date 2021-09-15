fn main() {
    let max = 10;
    let mut count = 1;
    let mut sum = 0;

    while count <= max {
        sum += count;
        count += 1;
    }
    println!("{}までの合計は{}です。", max, sum);
}
