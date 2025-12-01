fn let_demo() {
    println!("-----let Statements-----");
    // let PATTERN = EXPRESSION;
    let a = 5;
    let (x, y, z) = (1, 2, 3);
    // Compile Error: destructuring pattern doesn't match tuple length.
    // let (x, y) = (1, 2, 3);

    println!("-----Conditional if let Expressions-----");
    let favorite_color: Option<&str> = None;
    // let favorite_color: Option<&str> = Some("blue");
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    println!("-----while let Conditional Loops-----");
    let (tx1, rx) = std::sync::mpsc::channel();
    let tx2 = tx1.clone();
    std::thread::spawn(move || {
        for val in [1, 3, 5] {
            tx1.send(val).unwrap();
            tx2.send(val+1).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }
}

fn loops_demo() {
    let v = vec!['a', 'b', 'c'];

    for value in v.iter() {
        print!("{value} ");
    }
    println!();

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
    println!("v: {v:?}");
}

// Function parameters can also be patterns. 
fn print_tuple_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn print_individual_coordinates(&x:&i32, &y: &i32) {
    println!("Current location: ({x}, {y})");
}

fn fn_parameters_demo() {
    let point = (3, 5);
    print_tuple_coordinates(&point);
    print_individual_coordinates(&point.0, &point.1);
}

fn refutability_demo() {
    // let some_option_value: Option<i32> = None;
    let some_option_value: Option<i32> = Some(10);
    // Compile Error: `let` bindings require an "irrefutable pattern"
    // let Some(x) = some_option_value;
    let Some(x) = some_option_value else {
        return;
    };

    // Warning: This pattern will always match, so the `else` clause is useless
    // let x = 5 else { return; };
    let x = 5;

    let some_option = Some(5);
    match some_option {
        // refutable
        Some(x) => println!("Found a value: {}", x),
        // refutable
        None => println!("No value"),
    }
    match some_option {
        // refutable
        Some(x) => println!("Found a value: {}", x),
        // irrefutable
         _ => println!("Other case"),
    }

    let x = 10;
    // This syntax isn’t particularly useful and could be replaced with a simpler let statement.
    // match x {
    //     // irrefutable
    //     n => println!("The value is {}", n),
    // }
    let n = x;
    println!("The value is {}", n);
}

fn pattern_examples() {
    let a = 1;

    // Matching Literals
    match a {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Multiple Patterns
    match a {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values with ..=
    let aa = 'a';
    match a {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    match aa {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let b = Some(5);
    let c = 10;

    // Matching Named Variables
    match b {
        Some(50) => println!("Got 50"),
        Some(c) => println!("Matched, c = {c}"),
        _ => println!("Default case, b = {b:?}"),
    }
    println!("at the end: b = {b:?}, c = {c}");

    // Destructuring to Break Apart Values
    // Destructuring Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: d, y: e } = p;
    println!("d: {d:?}, e: {e:?}");
    let Point { x, y } = p;
    println!("x: {x:?}, y: {y:?}");

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructuring Enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColorRGB(i32, i32, i32),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColorRGB(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColorRGB(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        _ => (),
    }

    //Destructuring Nested Structs and Enums
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {feet}, inches: {inches}, x: {x}, y: {y}");

    // Ignoring Values in a Pattern

    // An Entire Value with_
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }
    foo(3,4);

    // Parts of a Value with a Nested _
    let mut setting_value = Some(5);
    // let new_setting_value = Some(0);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(0)) => {
            println!("A setting value of 0 is not allowed.");
        },
        (Some(5), Some(_)) => {
            println!("Default value 5 is being used. Can't overwrite.");
        },
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // An Unused Variable by Starting Its Name with _
    let _x = 5;

    let s = Some(String::from("Hello!"));
    // '_' doesn't bind.
    if let Some(_)= s {
        println!("found a string");
    }
    println!("{s:?}");
    // '_s' binds the value to the variable.
    if let Some(_s)= s {
        println!("found a string");
    }
    // Compile Error: value borrowed after partial move.
    // println!("{s:?}");

    // Remaining Parts of a Value with ..
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {x}"),
        // unreachable pattern
        // Point3D { x, y, z } => println!("x is {x}, y is {y}, z is {z}"),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
    match numbers {
        // Compile Error: '..' can only be used once per tuple pattern
        // (.., second, ..) => {
        //     println!("Some numbers: {second}")
        // },
        (_, second, ..) => {
            println!("Some numbers: {second}")
        }
    }

    // Extra Conditionals with Match Guards
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // let x = Some(5);
    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = true;

    match x {
        // '|' is 'or' operator to specify multiple pattenrs.
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ Bindings
    // The 'at' operator enables holding a value while simultaneously testing that value for a pattern match.
    enum Greeting {
        Hello { id: i32 },
    }
    let msg = Greeting::Hello { id: 5 };
    match msg {
        Greeting::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        Greeting::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Greeting::Hello { id } => println!("Found some other id: {id}"),
    }
}

pub fn run() {
    // let_demo();
    // loops_demo();
    // fn_parameters_demo();
    // refutability_demo();
    pattern_examples();
}