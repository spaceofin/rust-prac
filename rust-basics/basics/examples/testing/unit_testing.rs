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
}

pub fn unit_testing_demo() {

}