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

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    // tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn describe_color(color: Color) {
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
    }
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

fn match_destructuring() {
    println!("----------tuples----------");
    let triple = (1, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {
        // destructure
        (1, a, b) => println!("First is `1`, `a` is {:?}, and `b` is {:?}", a, b),
        // `..` can be used to ignore the rest of the tuple
        (2, ..)  => println!("First is `2` and the rest doesn't matter"),
        (.., 3)  => println!("last is `3` and the rest doesn't matter"),
        (4, .., 6)  => println!("First is `4`, last is `6`, and the rest doesn't matter"),
        // `_` means don't bind the value to a variable
        _      => println!("It doesn't matter what they are"),
    }

    println!("----------arrays/slices----------");
    let array_1 = [3, -2, 6];
    match array_1 {
        // Binds the second and the third elements to the respective variables
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        // Single values can be ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),
        // bind some and ignore the rest
        [first, ..] => println!(
            "array[0] = {} and all the other ones were ignored",
            first
        ),
    }
    let array_2 = [3, -4, 6, 9, -10];
    match array_2 {
        // ..
        [5, second, ..] => println!(
            "array[0] = 5, array[1] = {} and all the other ones were ignored",
            second
        ),
        // @ ..
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }

    println!("----------enums----------");
    println!("What color is it?");
    describe_color(Color::RGB(1221, 17, 40));
    println!("What color is it?");
    describe_color(Color::Red);
    println!("What color is it?");
    describe_color(Color::CMYK(0, 100, 100, 0));

    println!("----------structs----------");
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
        // and you can also ignore some variables:
        Foo { x, .. } => println!("x = {:?}, we don't care about y", x),
    }
    match foo {
        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 3, x: i } => println!("y is 2, i = {:?}", i),
        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };
    // You do not need a match block to destructure structs:
    let Foo { x : x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");
    // Destructuring works with nested structs as well:
    struct Bar {
        foo: Foo,
    }
    let bar = Bar { foo: faa };
    let Bar { foo: Foo { x: nested_x, y: nested_y } } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}

pub fn conditionals_demo() {
    // if_else_example();
    // match_example();
    match_destructuring();
}