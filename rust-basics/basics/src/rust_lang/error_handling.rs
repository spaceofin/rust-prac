fn trigger_panic() {
    // unrecoverable error
    panic!("This is a panic!");
}

fn induce_panic() {
    let v = vec![1, 2, 3];

    // Panic: index out of bounds.
    v[99];
}

fn recoverable_error() {
    use std::fs::File;
    use std::io::ErrorKind;
    
    let greeting_file_result = File::open("hello.txt");

    match greeting_file_result {
        Ok(file) => {
            println!("File exists!");
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => { 
                println!("'hello.txt' file not found."); 
            },
            _ => { 
                panic!("Problem opening the 'hello.txt' file: {error:?}"); 
            },
        }
    };
    
    let welcome_file_result = File::open("hi.txt");

    let welcome_file = welcome_file_result.unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("'hi.txt' file not found."); 
        } else {
            panic!("Problem opening the 'hi.txt' file: {error:?}");
        } 
    });
}

fn shortcuts_for_panic() {
    use std::fs::File;

    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!(""),
    // };

    // This is a shortcut for the match expression above.
    let greeting_file = File::open("hello.txt").unwrap();

    // This is a shortcut for the match expression with a custom panic message.
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
}

pub fn run() {
    // trigger_panic();
    // induce_panic();
    // recoverable_error();
    shortcuts_for_panic();
}