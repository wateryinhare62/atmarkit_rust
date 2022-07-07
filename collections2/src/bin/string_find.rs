fn main() {
    let english = String::from("Hello, world!");
    let japanese = String::from("こんにちは、世界！");
    let mut res = english.find('w');
    if res == None {
        println!("見つかりません。");
    } else {
        println!("{} バイト目で見つかりました！", res.unwrap());
    }
    res = japanese.find("世界");
    if res == None {
        println!("見つかりません。");
    } else {
        println!("{} バイト目で見つかりました！", res.unwrap());
    }
}
