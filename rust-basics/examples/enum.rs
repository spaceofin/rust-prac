// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use std::io::{self, Write};

enum WebEvent {
     // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // Paste(&'static str),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn _print_webevents() {
        let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    // let pasted = WebEvent::Paste("my text");
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}


fn get_char_input() -> char {
    loop {
    print!("Please enter a single character: ");
    // Flush the output buffer
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Trim whitespace including newline
    let trimmed = input.trim();
    if let Some(c) = trimmed.chars().next() {
            return c;
        } else {
            println!("No input detected, please try again.");
        }
    }
}

fn get_string_input() -> String {
    loop {
        print!("Please enter a string: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim whitespace and return String
        let trimmed = input.trim(); 
        if !trimmed.is_empty() {    
            return trimmed.to_string();
        } else {
            println!("Input cannot be empty. Please try again.");
        }
    }
}

fn handle_webevents() {
    println!("Select an event by number:");
    println!("1: PageLoad");
    println!("2: PageUnload");
    println!("3: KeyPress");
    println!("4: Paste");
    println!("5: Click");
    println!("6: Exit");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let choice = input.trim().parse::<u32>().unwrap_or(0);

    let event = match choice {
        1 => WebEvent::PageLoad,
        2 => WebEvent::PageUnload,
        3 => {
            let c = get_char_input();
            WebEvent::KeyPress(c)
        },
        4 => {
           let s = get_string_input();
            WebEvent::Paste(s)
        },
        5 => {
            WebEvent::Click { x: 10, y: 15 }
        },
        6 => {
            println!("Exiting...");
            return;
        }
        _ => {
            println!("Invalid choice.");
            return;
        }
    };

    inspect(event);
}


enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

mod enums;

fn print_stage_level_and_role() {
    //use crate::Stage::{Beginner, Advanced};
    use Stage::{Beginner, Advanced};
    //use crate::Role::*;
    use Role::*;
    use enums::example_enums::Level::*;

    let stage = Beginner;
    // Equivalent to `Role::Student`.
    let role = Student;
    let level = Medium;

    match stage {
        // Note the lack of scoping because of the explicit `use` above.
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // Note again the lack of scoping.
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }

    match level {
        Easy => println!("Easy level: Let's get started!"),
        Medium => println!("Medium level: Challenge yourself!"),    
        Hard => println!("Hard level: Brace yourself!"),    
    }
}


fn main(){
    //print_webevents();
    //handle_webevents()
    print_stage_level_and_role();
}