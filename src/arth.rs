pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero error".to_string())
    } else {
        Ok(a / b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 4.0), 6.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(3.0, 2.0), 1.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3.0, 4.0), 12.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
    }

    #[test]
    #[should_panic(expected = "Division by zero error")]
    fn test_divide_by_zero() {
        divide(10.0, 0.0).unwrap();
    }
}
