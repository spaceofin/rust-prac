use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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

fn channel_basic() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // Compile Error: value borrowed after move
        // println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");


}

fn recv_and_try_recv() {
    let (tx, rx) = mpsc::channel();

    let try_received = rx.try_recv();
    match rx.try_recv() {
        Ok(v) => println!("Got try: {v}"),
        Err(mpsc::TryRecvError::Empty) => {
            println!("No message yet");
        }
        Err(mpsc::TryRecvError::Disconnected) => {
            println!("Channel closed");
        }
    }

    // // recv() blocks here because the channel is empty and the sender is still alive.
    // let received_before_send = rx.recv().unwrap();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

fn send_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

fn mpsc() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("five"),
            String::from("six"),
            String::from("seven"),
            String::from("eight"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

fn mpsc_with_interleaving() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        for val in 0..100 {
            tx.send(format!("Val: {}", val)).unwrap();
            thread::yield_now();
        }
    });

    thread::spawn(move || {
        for val in 100..200 {
            tx1.send(format!("Val: {}", val)).unwrap();
            thread::yield_now(); 
        }
    });

    for received in rx {
        println!("{received}");
    }
}


fn spawn_multiple_senders_interleaving() {
    let (tx, rx) = mpsc::channel();

    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 0..3 {
                tx.send(format!("sender {id}: {i}")).unwrap();
                thread::yield_now(); 
            }
        });
    }

    drop(tx); 

    for msg in rx {
        println!("{msg}");
    }
}

pub fn run() {
    // spawn_and_main();
    // spawn_with_join();
    // spawn_with_move();
    // channel_basic();
    // recv_and_try_recv();
    // send_multiple_values();
    // mpsc();
    // mpsc_with_interleaving();
    spawn_multiple_senders_interleaving();
}