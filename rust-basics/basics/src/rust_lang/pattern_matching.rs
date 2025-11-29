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

pub fn run() {
    // let_demo();
    // loops_demo();
    // fn_parameters_demo();
    refutability_demo();
}