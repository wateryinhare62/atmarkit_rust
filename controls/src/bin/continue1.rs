fn main() {
    let scores = [90, 20, 100, 40, 60];
    let mut i = 5;
    while i > 0 {
        i -= 1;
        if scores[i] == 100 {
            continue;
        }
        println!("満点でない点数{}が要素{}にありました。", scores[i], i);
    }
}