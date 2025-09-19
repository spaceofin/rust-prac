use std::thread;

const NTHREADS: usize = 10;

pub fn threads_demo() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    let orders = [
    "first", "second", "third", "fourth", "fifth",
    "sixth", "seventh", "eighth", "ninth", "tenth"
    ];

    for i in 0..NTHREADS {
        let order = orders[i];
        // Spin up another thread
        children.push(thread::spawn(move || {
            print!("[thread {}] ", i);
            println!("This is the {} thread", order); 
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}