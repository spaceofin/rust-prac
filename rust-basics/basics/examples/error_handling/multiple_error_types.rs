use std::num::ParseIntError;

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
}

pub fn multiple_error_types_demo() {
    multiple_error_types_examples();
}