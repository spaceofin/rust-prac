use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
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

fn compare_rc_refcell_and_arc_mutex() {
    println!("-----Reference Counted Interior Mutability-----");
    let shared_val = Rc::new(RefCell::new(10));
    let rc1 = Rc::clone(&shared_val);
    
    println!("Reference Count: {}", Rc::strong_count(&shared_val));
    println!("[Before Mutation] shared_val: {shared_val:?}, rc1: {rc1:?}");
    let mut ref_mut = rc1.borrow_mut();
    *ref_mut += 10;
    println!("ref_mut: {ref_mut}");
    println!("[After Mutation] shared_val: {shared_val:?}, rc1: {rc1:?}");
    drop(ref_mut);
    println!("[After ref_mut drop] shared_val: {shared_val:?}, rc1: {rc1:?}");

    println!("-----Atomic Reference Counted Interior Mutability (Thread-Safe, Lock-Based)-----");
    let atomic_shared_val = Arc::new(Mutex::new(10));
    let rc1 = Arc::clone(&atomic_shared_val);
    let rc2 = Arc::clone(&atomic_shared_val);
    println!("Reference Count: {}", Arc::strong_count(&atomic_shared_val));
    let handle1 = thread::spawn(move || {
        let mut val = rc1.lock().unwrap();
        *val += 10;
        println!("[Thread1] Value updated to {val}");
    });
    println!("Reference Count After Spawning Thread1: {}", Arc::strong_count(&atomic_shared_val));

    let handle2 = thread::spawn(move || {
        let internal_clone = Arc::clone(&rc2);
        println!("[Thread2] Reference Count: {}", Arc::strong_count(&internal_clone));
        *internal_clone.lock().unwrap() += 10;
    });
    println!("Reference Count After Spawning Thread1: {}", Arc::strong_count(&atomic_shared_val));

    println!("[Parent Thread] val Before joins: {atomic_shared_val:?}");

    handle1.join().unwrap();
    println!("Reference Count After handle1 join: {}", Arc::strong_count(&atomic_shared_val));
    handle2.join().unwrap();
    println!("Reference Count After handle2 join: {}", Arc::strong_count(&atomic_shared_val));

    println!("[Parent Thread] val After joins: {atomic_shared_val:?}");
}

pub fn run() {
    // compare_rc_and_arc();
    compare_rc_refcell_and_arc_mutex();
}