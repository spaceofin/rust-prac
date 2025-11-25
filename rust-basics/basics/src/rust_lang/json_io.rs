use std::io::{self, Write};
use basics::commands::json_io::{save_json,load_json};

pub fn run() {
    loop {
        println!();
        print!("-----Enter Command Number-----\n1. save | 2. load | 3. exit > ");

        io::stdout().flush().unwrap(); 

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice = input.trim().parse::<u32>();

        println!();
        match choice {
            Ok(1) => {
                println!("-----Enter Content To Save-----");
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();
                let content = content.trim().to_string();

                println!();
                match save_json(content) {
                    Ok(_) => println!("Save successful!"),
                    Err(e) => eprintln!("Save failed: {}", e),
                }
            }
            Ok(2) => {
                match load_json() {
                    Ok(content) => println!("-----Loaded content-----------\n\n{}\n\n------------------------------", content),
                    Err(e) => eprintln!("Load failed: {}", e),
                }
            }
            Ok(3) => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid input. Please enter 1, 2, or 3.");
            }
        }
    }
}