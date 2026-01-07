use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn compare_rc_and_arc() {
    println!("-----Rc(Single-threaded Shared Ownership) - Non Atomic-----");
    let rc_val = Rc::new(10);
    let rc1 = Rc::clone(&rc_val);

    // Compile Error: `Rc<i32>` cannot be sent between threads safely.
    // thread::spawn(move || {
    //     println!("rc1 in thread: {}", rc1);
    // });

    let rc_owners = vec![rc1, Rc::clone(&rc_val)];
    println!("rc_owners: {rc_owners:?}");
    println!("Rc count: {}", Rc::strong_count(&rc_val));

    println!("-----Arc(Multi-threaded Shared Ownership) - Atomic-----");
    let arc_val = Arc::new(10);
    let arc1 = Arc::clone(&arc_val);

    // The 'move' keyword transfers ownership of 'arc1' into the child thread.
    let handle = thread::spawn(move || {
        println!("[Child Thread] Arc count: {}", Arc::strong_count(&arc1));
    });

    // Compile Error: `arc` value used here after move.
    // let arc_owners = vec![arc1, Arc::clone(&arc_val)];

    println!("[Parent Thread] Arc count Before Join: {}", Arc::strong_count(&arc_val));

    handle.join().unwrap();

    println!("[Parent Thread] Arc count After Join: {}", Arc::strong_count(&arc_val));

    println!("arc_val: {arc_val}");

    // Compile Error: `arc1` value borrowed here after move.
    // println!("arc1: {arc1}");
}

pub fn run() {
    compare_rc_and_arc();
}