// 注意：このファイルはコンパイルエラーとなります
mod graphics {
    mod calc {
        fn get_x() {}
        fn get_y() {}
    }
    mod draw {
        fn point() {}
        fn line() {}
        fn triangle() {}
        fn square() {}
    }
}

fn main() {
    crate::graphics::calc::get_x();
    graphics::draw::point();
}
