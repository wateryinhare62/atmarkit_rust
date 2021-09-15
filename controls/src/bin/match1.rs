fn main() {
    let letter = 'S';
    let str = match letter {
        'Z' => "アルファベット最後の文字",
        'S' | 'M' | 'L' => "サイズを表すアルファベット",
        '0'..='9' | 'A'..='F' => "16進数で使う文字",
        _ => "いずれでもない文字",
    };
    println!("{}は{}です。", letter, str);
}
