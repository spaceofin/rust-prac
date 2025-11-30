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

}

pub fn run() {
    // let_demo();
    // loops_demo();
    // fn_parameters_demo();
    // refutability_demo();
    pattern_examples();
}