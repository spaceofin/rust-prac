use std::fs::File;
use std::io::{Read, ErrorKind, Write};
use std::path::Path;

fn open_greeting_file() {
    let base_path = Path::new(file!())
        .parent()
        .expect("Could not get parent directory");

    let file_path = base_path.join("hello.txt");

    let greeting_file_result = File::open(&file_path);

    let _greeting_file = match greeting_file_result {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read file"); 
            println!("{}", contents);
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_path) {
                Ok(mut fc) => {
                fc.write_all(b"Rust Rust Rust!").expect("Failed to write default content");
                println!("New file created: {}", file_path.display());
                fc
                }
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}

pub fn trigger_recovarable_errors() {
    open_greeting_file();
}