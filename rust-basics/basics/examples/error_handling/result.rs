use std::panic;
use std::num::ParseIntError;

fn multiply_and_print_result(first_number_str: &str, second_number_str: &str) {
  let result = panic::catch_unwind(|| {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
  });

  match result {
    Ok(val) => println!("The result of multiplication is {}",val),
    Err(_) => println!("Panic occurred during multiplication!"),
  }
}

fn multiply_with_match(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number)  => {
            match second_number_str.parse::<i32>() {
                Ok(second_number)  => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn multiply_with_combinators(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print_result(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {:?}", e),
    }
}

pub fn result_demo() {
  println!("---------------multiply and print---------------");
  multiply_and_print_result("10","2");
  multiply_and_print_result("t","2");

  println!("---------------multiply with match---------------");
  let twenty = multiply_with_match("10", "2");
  print_result(twenty);
  let tt = multiply_with_match("t", "2");
  print_result(tt);

  println!("---------------multiply with combinators---------------");
  let twenty = multiply_with_combinators("10", "2");
  print_result(twenty);
  let tt = multiply_with_combinators("t", "2");
  print_result(tt);
}