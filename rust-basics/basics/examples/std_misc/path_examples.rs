use std::path::{Path, PathBuf};

fn path_basic() {
    // Create a `Path` from an `&'static str`
    let path: &Path = Path::new(".");
    println!("path: {}", path.display());

    // `join` merges a path with a byte container using the OS specific separator, and returns a `PathBuf`
    let mut new_path: PathBuf = path.join("a").join("b");

    let path_b = PathBuf::from(r".\a\b\");
    if new_path == path_b {
        println!("same path!");
    } else {
        println!("different path!");
    }
    
    // `push` extends the `PathBuf` with a `&Path`
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    println!("new path is {}", new_path.to_str().unwrap());

    // `set_file_name` replaces the last path component with the given file name
    new_path.set_file_name("package.tgz");

    // Convert the `PathBuf` into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

pub fn path_examples() {
    path_basic();
}