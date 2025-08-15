use simple_logger::SimpleLogger;
use log::{LevelFilter, info, warn, error, debug, trace};

fn task_1() {
    info!("Task1: loading data");
    debug!("Task1: debug info"); // not printed
    if let Err(_) = do_something() {
        warn!("Task1: something went wrong");
    }
}

fn task_2() {
    info!("Task2: processing data");
    trace!("Task2: trace info"); // not printed
    error!("Task2: critical error encountered");
}

fn do_something() -> Result<(), ()> {
    Err(()) // Assume failure
}

pub fn simple_logger_demo() {
    println!("----------simple logger demo----------");

    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    info!("Starting the program...");

    task_1();
    task_2();

    info!("Program finished.");
}