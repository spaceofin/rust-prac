use predicates::prelude::*;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; 
    // crate for test-only use. Cannot be used in non-test code.

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_intentional_fail() {
        assert_eq!(add(2, 3), 7);
    }

    #[test]
    fn test_numbers_basic_assert() {
        let number1 = 42;
        let number2 = 52;

        assert!(number1 > 40 && number1 < 50, "Expected number between 40 and 50, got {}", number1);
        assert!(number2 > 40 && number2 < 50, "Expected number between 40 and 50, got {}", number2);
    }

    #[test]
    fn test_numbers_predicates() {
        let number1 = 43;
        let number2 = 53;

        let is_valid_number = predicate::function(|&number: &i32| number > 40 && number < 50);

        assert!(is_valid_number.eval(&number1), "Expected number between 40 and 50, got {}", number1);
        assert!(is_valid_number.eval(&number2), "Expected number between 40 and 50, got {}", number2);
    }
}