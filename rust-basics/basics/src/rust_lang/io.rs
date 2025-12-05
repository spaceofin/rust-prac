use std::{env, fs, process, error::Error};
use std::path::{Path, PathBuf};
use basics::library::search::{search_lines, search_lines_case_insensitive};

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
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self { query, file_path, ignore_case })
    }
}

pub fn run_with_config(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With text:\n{contents}");
    println!("\n-----contents-----\n{contents}");
    println!("\n-----search results-----");
    let results = if config.ignore_case {
        search_lines_case_insensitive(&config.query, &contents)
    } else {
        search_lines(&config.query, &contents)
    };
    for line in results {
        println!("found: {line}");
    }
    Ok(())
}

fn config_demo() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // println!("config: {config:?}");
    println!("Searching for '{}'", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run_with_config(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search_lines(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."], search_lines_case_insensitive(query, contents)
        );
    }
}

pub fn run() {
    // read_args();
    // save_two_args();
    // save_args();
    // read_file();
    config_demo();
}