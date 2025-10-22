use std::num::ParseIntError;
use std::fmt;
use std::error::{self, Error};

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

fn double_first_qmark(vec: &Vec<&str>) -> BoxResult<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print_box_result(result: BoxResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

type DoubleKindResult<T> = std::result::Result<T, DoubleErrorKind>;

#[derive(Debug)]
enum DoubleErrorKind {
    EmptyVec,
    Parse(ParseIntError),
}

impl fmt::Display for DoubleErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleErrorKind::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            DoubleErrorKind::Parse(..) =>
                write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for DoubleErrorKind {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleErrorKind::EmptyVec => None,
            // The underlying ParseIntError is returned as a &dyn Error automatically.
            DoubleErrorKind::Parse(ref e) => Some(e),
        }
    }
}

// Converts ParseIntError into DoubleError; used automatically by `?`.
impl From<ParseIntError> for DoubleErrorKind {
    fn from(err: ParseIntError) -> DoubleErrorKind {
        DoubleErrorKind::Parse(err)
    }
}

fn double_first_kind(vec: &Vec<&str>) -> DoubleKindResult<i32> {
    let first = vec.first().ok_or(DoubleErrorKind::EmptyVec)?;
    // Here we implicitly use the `ParseIntError` implementation of `From` in order to create a `DoubleError`.
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print_double_kind_result(result: DoubleKindResult<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        },
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

    println!("----------double_first_qmark----------");
    print_box_result(double_first_qmark(&numbers));
    print_box_result(double_first_qmark(&empty));
    print_box_result(double_first_qmark(&strings));

    println!("----------double_first_kind----------");
    print_double_kind_result(double_first_kind(&numbers));
    print_double_kind_result(double_first_kind(&empty));
    print_double_kind_result(double_first_kind(&strings));
}

pub fn multiple_error_types_demo() {
    multiple_error_types_examples();
}