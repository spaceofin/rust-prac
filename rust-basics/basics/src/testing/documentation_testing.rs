
/// Code blocks starts with triple backquotes
/// and have implicit `fn main()` inside.
/// 
/// ```
/// let result = basics::testing::documentation_testing::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// # Examples
///
/// ```
/// let result = basics::testing::documentation_testing::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// ```rust,should_panic
/// // panics on division by zero
/// basics::testing::documentation_testing::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}

/// Using hidden `try_main` in doc tests.
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compilable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// let res = basics::testing::documentation_testing::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { try_main().unwrap(); }
/// 
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}