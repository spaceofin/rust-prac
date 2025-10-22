use std::num::ParseIntError;
use std::fmt;
use std::error;

fn double_first_with_two_errors(vec: &Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generator error 1
    2 * first.parse::<i32>().unwrap() // Generator error 2
}

fn double_first(vec: &Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn double_first_safe(vec: &Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.transpose()
}

type DoubleResult<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first_with_custom_error(vec: &Vec<&str>) -> DoubleResult<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print_double_result(result: DoubleResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

type BoxResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first_box(vec: &Vec<&str>) -> BoxResult<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // Converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Converts to Box
                .map(|i| 2 * i)
        })
}

fn print_box_result(result: BoxResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}


fn multiple_error_types_examples() {
    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("----------double_first_with_two_errors----------");
    println!("The first doubled is {}", double_first_with_two_errors(&numbers));
    // Error 1: called `Option::unwrap()` on a `None` value
    // println!("The first doubled is {}", double_first_with_two_errors(&empty));
    // Error 2: called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
    // println!("The first doubled is {}", double_first_with_two_errors(&strings));

    println!("----------double_first----------");
    println!("The first doubled is {:?}", double_first(&numbers));
    // Error 1: the input vector is empty
    println!("The first doubled is {:?}", double_first(&empty));
    // Error 2: the element doesn't parse to a number
    println!("The first doubled is {:?}", double_first(&strings));

    println!("----------double_first_safe----------");
    println!("The first doubled is {:?}", double_first_safe(&numbers));
    println!("The first doubled is {:?}", double_first_safe(&empty));
    println!("The first doubled is {:?}", double_first_safe(&strings));

    println!("----------double_first_with_custom_errors----------");
    print_double_result(double_first_with_custom_error(&numbers));
    print_double_result(double_first_with_custom_error(&empty));
    print_double_result(double_first_with_custom_error(&strings));

    println!("----------double_first_box----------");
    print_box_result(double_first_box(&numbers));
    print_box_result(double_first_box(&empty));
    print_box_result(double_first_box(&strings));
}

pub fn multiple_error_types_demo() {
    multiple_error_types_examples();
}