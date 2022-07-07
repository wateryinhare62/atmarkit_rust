trait Area {
    fn calc(&self) -> f64;
}

struct Triangle {
    base: f64,
    height: f64,
}

struct Trapezoid {
    top_base: f64,
    bottom_base: f64,
    height: f64,
}

impl Area for Triangle {
    fn calc(&self) -> f64 {
        (self.base * self.height) / 2.0
    }
}

impl Area for Trapezoid {
    fn calc(&self) -> f64 {
        (self.top_base + self.bottom_base) * self.height / 2.0
    }
}

fn main() {
    let triangle = Triangle {base: 10.0, height: 20.0};
    let trapezoid = Trapezoid {top_base: 10.0, bottom_base: 20.0, height: 10.0};
    println!("Area of triangle: {}", triangle.calc());
    println!("Area of trapezoid: {}", trapezoid.calc());
}