fn main()
{
    let t = (2, 3.14, 0);
    let (a, b, c) = t;
    println!("a={}, b={}, c={}", a, b, c);
    let x = t.0;
    let y = t.1;
    let z = t.2;
    println!("x={}, y={}, z={}", x, y, z);
}
