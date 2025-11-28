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

pub fn run() {
    let_demo();
}