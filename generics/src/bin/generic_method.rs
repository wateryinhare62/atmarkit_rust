struct Range<T> {
    min: T,
    max: T,
    step: T,
    current: T,
}

impl<T> Range<T> {
    fn new(min: T, max: T, step: T, current: T) -> Self {
        Range {min, max, step, current}
    }
}

fn main() {
    let int_range = Range::new(1, 10, 1, 5);
    let float_range = Range::new(1.0, 100.0, 0.1, 0.0);
    println!("min: {}, max: {}, step: {}, current:{}", 
        int_range.min, int_range.max, int_range.step, int_range.current);
    println!("min: {}, max: {}, step: {}, current:{}", 
        float_range.min, float_range.max, float_range.step, float_range.current);
}
