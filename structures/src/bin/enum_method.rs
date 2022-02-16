fn main() {
    enum Shape {
        None,
        Line {x1: i32, y1: i32, x2: i32, y2: i32},
        Circle {x: i32, y: i32, r: i32},
        Rect(i32, i32, i32, i32),
        Text(String),
    }

    impl Shape {
        fn draw(self: &Self) {
            match self {
                Shape::None => println!("None"),
                Shape::Line {x1, y1, x2, y2} => 
                    println!("Line: ({}, {})-({}, {})", x1, y1, x2, y2),
                Shape::Circle {x, y, r} => println!("Circle: ({}, {}), {}", x, y, r),
                Shape::Rect(x1, y1, x2, y2) => 
                    println!("Rect: ({}, {})-({}, {})", x1, y1, x2, y2),
                Shape::Text(s) => println!("Text: {}", s),
            }
        }
    }
    let n = Shape::None;
    let l = Shape::Line {x1: 100, y1: 200, x2: 300, y2: 400};
    let c = Shape::Circle {x: 100, y: 200, r: 50};
    let r = Shape::Rect(100, 200, 300, 400);
    let t = Shape::Text(String::from("Hello."));
    n.draw();
    l.draw();
    c.draw();
    r.draw();
    t.draw();
}