use std::thread;
use std::time::Duration;

pub fn threads_demo() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    println!("__________sleep 3s start__________");
    thread::sleep(Duration::from_millis(3000));
    println!("__________sleep 3s end__________");

    let mut handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();

    println!("__________sleep 3s start__________");
    thread::sleep(Duration::from_millis(3000));
    println!("__________sleep 3s end__________");
    
    
    handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("main thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }
}