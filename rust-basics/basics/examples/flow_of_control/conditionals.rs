use std::io::{self, Write};

fn if_else_example () {
    let n_arr: [i32; 4] = [-5,5,-10,10];

    for n in n_arr {
        if n < 0 {
            print!("{} is negative", n);
        } else if n > 0 {
            print!("{} is positive", n);
        } else {
            print!("{} is zero", n);
        }

        let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n // i32
        } else {
            println!(", and is a big number, halve the number");
            n / 2 // i32 as well
        };// Don't forget to put a semicolon here! All `let` bindings need it.
        println!("{} -> {}", n, big_n);
    }
}

fn match_example () {
    println!("\n----------match number----------");
    loop{
        print!("Enter a positive number <= 20(type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting the program.");
            break; // exit the loop
        }

        let number: i32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, not an integer.");
                continue; 
            }
        };

        if number < 1 || number > 20 {
             println!("Please enter a positive number between 1 and 20");
            continue;
        }
        
        println!("Tell me about {}...", number);
        match number {
            // Match a single value
            1 => println!("One!"),
            // Match several values
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => println!("This is a prime"),
            // Match an inclusive range
            13..=19 => println!("A teen"),
            // Handle the rest of cases
            _ => println!("Ain't special"),
        }
        println!();
    }
}

pub fn conditionals_demo() {
    if_else_example();
    match_example();
}