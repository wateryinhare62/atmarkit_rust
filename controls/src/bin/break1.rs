fn main() {
    let scores = [90, 20, 100, 40, 60];
    let mut i = 0;
    let f = loop {
        if scores[i] == 100 {
            break i;
        }
        i += 1;
    };
    println!("満点が要素{}にありました。", f);
}