use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rand::Rng;

static NTHREADS: i32 = 10;

pub fn channels_demo() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            let sleep_ms = rand::thread_rng().gen_range(1000..5000);
            thread::sleep(Duration::from_millis(sleep_ms));
            let sleep_s = sleep_ms as f32 / 1000.0;

            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();
            // Sending is a non-blocking operation
            println!("thread {} finished after {:.3}s", id, sleep_s);

        });
        children.push(child);
    }
    drop(tx);

    // all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        match rx.recv() {
            Ok(msg) => {
                println!("Main thread received: {}", msg);
                ids.push(msg);
            }
            Err(_) => {
                println!("The channel has been closed.");
                break;
            }
        }
    }
    
    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
}