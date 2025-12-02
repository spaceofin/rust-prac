use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process;

fn read_args() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}

fn save_two_args() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}

fn save_args() {
    let args: Vec<String> = env::args().collect();

    for (i, arg) in args.iter().enumerate().skip(1) {
        println!("Arg {}: {}", i, arg);
    }
}

fn get_base_path() -> PathBuf {
    let source_file = file!();
    Path::new(source_file).parent().unwrap().to_path_buf()
}

fn read_file() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_name = &args[2];

    let base_path = get_base_path();
    let path = base_path.join("files").join(&file_name);
    
    println!("In file {path:?}");
    println!();

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // fn new(args: &[String]) -> Self {
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
    //     Self { query, file_path }
    // }
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Self { query, file_path })
    }
}

fn run_with_config(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file"); 
    println!("With text:\n{contents}");
}

fn config_demo() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // println!("config: {config:?}");
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    run_with_config(config);
}

pub fn run() {
    // read_args();
    // save_two_args();
    // save_args();
    // read_file();
    config_demo();
}