// 注意：このファイルはコンパイルエラーとなります
pub mod graphics {
    mod calc {
        pub fn get_x() {}
        fn get_y() {}
    }
    pub mod draw {
        pub fn point() {}
        pub fn line() {}
        fn triangle() {}
        fn square() {}
    }
}

fn main() {
    crate::graphics::calc::get_x();
    graphics::draw::point();
    graphics::draw::square();
}
