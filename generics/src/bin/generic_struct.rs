struct Range<T> {
    min: T,
    max: T,
    step: T,
    current: T,
}

fn main() {
    let int_range = Range {min: 1, max: 10, step:1, current: -1};
    let float_range = Range {min: 1.0, max: 100.0, step: 0.1, current: -1.0};
    //let mixed_range = Range {min: 1.0, max: 10, step: 1, current: -1.0};
    println!("min: {}, max: {}, step: {}, current:{}", 
        int_range.min, int_range.max, int_range.step, int_range.current);
    println!("min: {}, max: {}, step: {}, current:{}", 
        float_range.min, float_range.max, float_range.step, float_range.current);
}
