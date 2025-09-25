use std::fs::{File, read_to_string};
use std::io::{prelude::*, self, BufRead};
use std::path::{Path, PathBuf};

fn get_base_path() -> PathBuf {
    let source_file = file!();
    Path::new(source_file).parent().unwrap().to_path_buf()
}

fn enter_filename() -> String {
    print!("Enter the file name in the files folder: ");
    io::stdout().flush().unwrap();
    
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read input");
    filename.trim().to_string()
}

fn open_example() {
    let filename = enter_filename();
    
    let base_path = get_base_path();
    let path = base_path.join("files").join(&filename);
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

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn create_example() {
    let base_path = get_base_path();
    let filename = enter_filename();

    let path = base_path.join("files").join(&filename);
    let display = path.display();

    if path.exists() { 
        println!("File {} already exists. It will be overwritten.", &filename); 
    } else { 
    println!("Creating a new file: {}", &filename);
    }

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}


fn read_lines_loop<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn read_lines_iter<P: AsRef<Path>>(path: P) -> Vec<String> {
    read_to_string(path) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_lines_examples() {
    let base_path: PathBuf = get_base_path();
    let filename: String = enter_filename();
    let path: PathBuf = base_path.join("files").join(&filename);
    
    let loop_result = read_lines_loop(&path);
    println!("read_lines_loop result:\n {:?}\n", loop_result);

    let iter_result = read_lines_iter(&path);
    println!("read_lines_iter result:\n {:?}\n", iter_result);

    println!("read_lines result:");
    if let Ok(lines) = read_lines(&path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}


pub fn file_io_demo() {
    // open_example();
    // create_example();
    read_lines_examples();
}