use std::env;

fn program_arguments_basic() {
    let args: Vec<String> = env::args().collect();

    println!("args: {:?}", &args);

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //  $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}",  number - 1 );
}

fn help() {
    println!("usage:
    match_args <string>
        Check whether given string is the answer.
    match_args {{increase|decrease}} <integer>
        Increase or decrease given integer by one.");
}

fn argument_parsing() {
    let args: Vec<String> = env::args().collect();

    println!("The first argument is the path that was used to call the program:\n\t{}", args[0]);
    println!("[Q] What is the answer to life, the universe, and everything?");
    println!("[Info] Use 'increase' or 'decrease' command with a number.");

    match args.len() {
        // no arguments passed
        1 => {
            println!("[!] No arguments were passed! Try passing some arguments!");
        },
        // one argument passed
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            }
        },
        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            // parse the command
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}

pub fn program_arguments_demo() {
    // program_arguments_basic();
    argument_parsing();
}