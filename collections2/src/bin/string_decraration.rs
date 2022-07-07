fn main() {
    let jan = String::from("January");
    let feb_utf8 = vec![0x46, 0x65, 0x62, 0x72, 0x75, 0x61, 0x72, 0x79];
    let feb = String::from_utf8(feb_utf8).unwrap();
    println!("{},{}", jan, feb);
}
