fn main() {
    enum Shape {
        None,
        Line {x1: i32, y1: i32, x2: i32, y2: i32},
        Circle {x: i32, y: i32, r: i32},
        Rect(i32, i32, i32, i32),
        Text(String),
    }

    let _n = Shape::None;
    let _l = Shape::Line {x1: 100, y1: 200, x2: 300, y2: 400};
    let _c = Shape::Circle {x: 100, y: 200, r: 50};
    let _r = Shape::Rect(100, 200, 300, 400);
    let _t = Shape::Text(String::from("Hello."));
}