// An integer division that doesn't `panic!`
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 { None }// Failure
    else { Some(dividend / divisor) } // Result is wrapped in a `Some` variant
}

fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient)
    }
}

fn option_example() {
    try_division(4, 2);
    try_division(1, 0);

    // Binding `None` to a variable needs to be type annotated
    // let a = None; // Compile error
    let _none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // Unwrapping a `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Unwrapping a `None` variant will `panic!`
    // println!("{:?} unwraps to {:?}", none, none.unwrap()); // Runtime error
}

mod checked {
    // Mathematical "errors" we want to catch
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            // result wrapped in `Ok`
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) -> Option<f64> {
    // This is a three level match pyramid!
    match checked::div(x, y) {
        Err(why) => {
            println!("div error: {:?}",why);
            return None;
            // panic!("{:?}", why),
        },
        Ok(ratio) => {
            println!("div result: {:?}",ratio);
            match checked::ln(ratio) {
                Err(why) => {
                    println!("ratio error: {:?}",why);
                    return None;
                    // panic!("{:?}", why),
                },
                Ok(ln) => {
                    println!("ln result: {:?}",ln);
                    match checked::sqrt(ln) {
                        Err(why) => {
                            println!("ln error: {:?}",why);
                            return None;
                            // panic!("{:?}", why),
                        },
                        Ok(sqrt) => {
                            println!("sqrt result: {:?}",sqrt);
                            Some(sqrt)
                        },
                    }
                },
            }
        },
    }
}

fn result_example() {
    println!();
    println!("___op output: {}", op(1.0, 10.0).map_or("failed".to_string(), |v| v.to_string()));
    println!();
    println!("___op output: {}", op(10.0, 10.0).map_or("failed".to_string(), |v| v.to_string()));
}


pub fn option_and_result_demo() {
    // option_example();
    result_example();
}