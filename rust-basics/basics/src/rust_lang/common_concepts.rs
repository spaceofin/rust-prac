use std::io;

fn variables_mutability () {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn constants() {
    println!("three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}

fn shadowing () {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    
    // shadowing
    let spaces_a = "   ";
    let spaces_a = spaces_a.len();

    let mut spaces_b = "   ";
    // Compile Error: expected '&str'
    // spaces_b = spaces_b.len();
}

fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!");
    
    // floating-point types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // numeric operations
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;

    // the boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation

    // the character type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // compound type
    // the tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // the array type
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);
    let first = a[0];
    let second = a[1];
    println!("a[0]: {}, a[1]: {}", first, second);
    let a = [3; 5];
    println!("a: {:?}", a);
}

fn array_element_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn functions() {
    another_function(5);
    print_labeled_measurement(5,'h');
}

fn statements_and_expressions() {
    // statement
    let y = 6;

    // statement don't return values
    // Compile Error: expected expression, found `let` statement
    // let x = (let y = 6);
    
    // A block is an expression
    let y = {
        let x = 3;
        x + 1
    }; 
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Compile Error: no `return` expression
// fn plus_two(x: i32) -> i32 {
//     x + 2;
// }

fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Compile Error: expected `bool`, found integer
    // if number {
    //     println!("number was three");
    // }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // Compile Error: if and else arms have value types that are imcompatible.
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");

    // Print `again!` forever...
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loop labels
    // loop labels must begin with a single quote.
    println!();
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        'inner: loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 1 {
                break 'inner;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with while
    println!();
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping Throuthe a Collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    // while construct to loop over the elements of a collection
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // more concise alternative: use a `for` loop
    println!();
    for element in a {
        println!("the value is: {element}");
    }
    // Reverse the range using `rev`
    println!();
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

pub fn run() {
    // This is a comment.
    println!("\n----------Variables Mutability----------");
    variables_mutability();
    println!("\n----------Constants----------");
    constants();
    println!("\n----------Shadowing----------");
    shadowing();
    println!("\n----------Data Types----------");
    data_types();
    // println!("\n----------Array Element Access----------");
    // array_element_access();
    println!("\n----------Functions----------");
    functions();
    println!("\n----------Statements and Expressions----------");
    statements_and_expressions();
    println!("\n----------Control Flow----------");
    control_flow();
}