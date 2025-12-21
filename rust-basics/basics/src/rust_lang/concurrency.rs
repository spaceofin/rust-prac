use std::thread;
use std::time::Duration;

fn spawn_and_main() {
    // spawned thread
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // main thread
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

fn spawn_with_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Calling join() here makes the main thread wait until the spawned thread finishes.
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn spawn_with_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // Compile Error: value borrowed after move
    // println!("v: {v:?}");

    handle.join().unwrap();
}

pub fn run() {
    // spawn_and_main();
    // spawn_with_join();
    spawn_with_move();
}