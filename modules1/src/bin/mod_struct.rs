// 注意：このファイルはコンパイルエラーとなります。
pub mod graphics {
    struct Point {
        pub x: i32, pub y: i32,
    }
    pub struct Rect {
        x: i32, y: i32, w: i32, h:i32, 
    }
}

fn main() {
    let p = graphics::Point {x: 100, y: 100};
    println!("Point = {}, {}", p.x, p.y);
    let r = graphics::Rect {x: 100, y: 100, w: 200, h: 300};
    println!("Rect = {}, {}, {}, {}", r.x, r.y, r.w, r.h);
}