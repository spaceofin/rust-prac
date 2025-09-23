use std::fs::File;
use std::io::{prelude::*, self};
use std::path::Path;

fn open_example() {

    print!("Enter the file name in the files folder: ");
    io::stdout().flush().unwrap();

    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read input");
    let filename = filename.trim();

    let source_file = file!();
    
    let base_path = Path::new(source_file).parent().unwrap();
    let path = base_path.join("files").join(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

pub fn file_io_demo() {
    open_example();
}