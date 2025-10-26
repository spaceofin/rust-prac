use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
#[cfg(target_family = "unix")]
use std::os::unix;
#[cfg(target_family = "windows")]
use std::os::windows;
use std::path::{Path, PathBuf};
use std::env;

// A simple implementation of `% cat path`
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A simple implementation of `% echo s > path`
fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

// A simple implementation of `% touch path` (ignore existing files)
fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn get_base_path() -> PathBuf {
    let source_file = file!();
    Path::new(source_file).parent().unwrap().to_path_buf()
}

pub fn filesystem_operations_demo() {

    let base_path = get_base_path();
    let a_path = base_path.join("a");

    println!("`mkdir a`");
    // Create a directory, returns `io::Result<()>`
    match fs::create_dir(&a_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }
    println!();

    println!("`echo hello > a/b.txt`");
    let b_path = base_path.join("a/b.txt");
    // The previous match can be simplified using the `unwrap_or_else` method
    echo("hello", &b_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!();

    println!("`mkdir -p a/c/d`");
    let d_path = base_path.join("a/c/d");
    // Recursively create a directory, returns `io::Result<()>`
    fs::create_dir_all(&d_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!();

    println!("`touch a/c/e.txt`");
    let e_path = base_path.join("a/c/e.txt");
    touch(&e_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!();

    println!("`ln -s ../b.txt a/c/b.txt`");
    let c_b_path = base_path.join("a/c/b.txt");
    // Create a symbolic link, returns `io::Result<()>`
    #[cfg(target_family = "unix")] {
        unix::fs::symlink("../b.txt", &c_b_path).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    #[cfg(target_family = "windows")] {
        windows::fs::symlink_file(r"..\b.txt", &c_b_path).unwrap_or_else(|why| {
            println!("! {:?}", why.to_string());
        });
    }
    println!();

    println!("`cat a/c/b.txt`");
    match cat(&c_b_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }
    println!();

    println!("`ls a`");
    // Read the contents of a directory, returns `io::Result<Vec<Path>>`
    match fs::read_dir(a_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }
    println!();

    println!("`rm a/c/e.txt`");
    let e_path = base_path.join("a/c/e.txt");
    // Remove a file, returns `io::Result<()>`
    fs::remove_file(&e_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!();

    println!("`rmdir a/c/d`");
    // Remove an empty directory, returns `io::Result<()>`
    fs::remove_dir(&d_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!();
}
