use std::fs::{File, read};
use std::io::{self, Read};

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

fn read_contents_from_file() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

fn read_contents_from_file_with_qmark() -> Result<String, io::Error> {
    // let mut file_result = File::open("hello.txt")?;
    let mut contents = String::new();
    File::open("hello.txt")?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn qmark_example() {
    match read_contents_from_file_with_qmark() {
        Ok(contents) => println!("file contents>\n{contents}"),
        Err(e) => eprintln!("error: {e}"),
    }

    match last_char_of_first_line("abc") {
        Some(c) => println!("last char: {c}"),
        None => println!("No character found."),
    }

    match last_char_of_first_line("") {
        Some(c) => println!("last char: {c}"),
        None => println!("No character found."),
    }
}

pub fn run() {
    // trigger_panic();
    // induce_panic();
    // recoverable_error();
    // shortcuts_for_panic();
    qmark_example();
}