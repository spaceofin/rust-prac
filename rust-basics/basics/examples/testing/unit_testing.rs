pub fn add(a: i32, b: i32) -> i32 { a + b }

// The purpose of this function is to fail in this example
fn bad_add(a: i32, b:i32) -> i32 { a - b }

fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    // Note thie useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let pos = 4.0;
        assert_eq!(sqrt(pos)?.powf(2.0), pos);

        let neg = -4.0;
        assert!(sqrt(neg).is_err());

        Ok(())
    }

        #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide-by-zero error")]
    fn test_divide_by_zero_panic_expected() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide by zero error")]
    fn test_divide_by_zero_panic_wrong_expected() {
        divide_non_zero_result(1, 0);
    }

    #[should_panic(expected = "Divide result is zero")]
    fn test_devide_result_zero_panic() {
        divide_non_zero_result(1, 10);
    }

    #[test]
    #[should_panic(expected = "result")]
    fn test_devide_result_zero_panic_substring() {
        divide_non_zero_result(1, 10);
    }

    #[test]
    #[should_panic = "Divide result is zero"] 
    fn test_devide_result_zero_panic_shorthand() {
        divide_non_zero_result(1, 10);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }
}