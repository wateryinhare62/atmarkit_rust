/// Adds two number given.
///
/// # Arguments
/// * `a` - 1st number
/// * `b` - 2nd number
///
/// # Examples
///
/// ```
/// let a = 5;
/// let b = 4;
/// assert_eq!(9, testings::adder2(a, b));
/// ```
pub fn adder2(a: i32, b:i32) -> i32 {
    a + b
}

pub fn adder2_bad(a: i32, b:i32) -> i32 {
    // 間違ったコード
    a + b + 1  
}

pub fn reciprocal(n: f64) -> f64 {
    if n == 0.0 {
        panic!("0の逆数は計算できません！");
    }
    1.0 / n
}

pub fn reciprocal_bad(n: f64) -> f64 {
    // 間違ったコード
    if n == 100.0 {
        panic!("0の逆数は計算できません！");
    }
    1.0 / n
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn add_success() {
        assert_eq!(crate::adder2(5, 4), 9)
    }
    #[test]
    fn add_fail() {
        assert_eq!(crate::adder2_bad(5, 4), 9)
    }
    #[test]
    #[should_panic]
    fn reciprocal_success() {
        crate::reciprocal(0.0);
    }
    #[test]
    #[should_panic]
    fn reciprocal_fail() {
        crate::reciprocal_bad(0.0);
    }
}
