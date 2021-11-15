fn main() {
    let sa = [1, 3, 2];
    let sb = sa;
    let ca = [String::from("a"), String::from("b")];
    let cb = ca;
    println!("sa[0]は{}、sb[0]は{}です。", sa[0], sb[0]);
    println!("ca[0]は{}、cb[0]は{}です。", ca[0], cb[0]);
}
