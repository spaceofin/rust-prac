use std::rc::Rc;
use std::time::Duration;
use std::sync::Arc;
use std::thread;

fn rc_examples() {
    let rc_examples = "Rc examples".to_string();
    {
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("--- rc_a is created ---");
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        
        {
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("--- rc_a is cloned to rc_b ---");

            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // Two 'Rc's are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }

    println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

    println!("--- rc_a is dropped out of scope ---");

    }
    // When `rc_a` is dropped, `rc_examples` is dropped together
    // This is because `rc_examples` already moved into `rc_a`
}

fn arc_examples() {
    // This variable declaration is where its value is specified.
    let apple = Arc::new("the same apple");

    for i in 0..10 {
        // Here there is no value specification 
        // as it is a pointer to a reference in the memory heap.
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("Thread {}: {:?}",i, apple);
            println!("Reference Count of apple: {}",Arc::strong_count(&apple));
            
        });

        // Compile Error: value borrowed here after move
        // println!("Reference Count of apple: {}",Arc::strong_count(&apple));
    }

    // Make sure all Arc instances are printed from spawned threads.
    thread::sleep(Duration::from_secs(1));

    println!("Reference Count of apple, after 'for' block: {}",Arc::strong_count(&apple));

    println!();
    println!("___ Spawning 10 threads with 1 second interval each ___");
    for i in 0..10 {
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            println!("Thread {}: {:?}",i, apple);
            println!("Reference Count of apple: {}",Arc::strong_count(&apple));
            
        });
        thread::sleep(Duration::from_secs(1));
    }
}

pub fn rc_and_arc_demo() {
    rc_examples();
    println!();
    arc_examples();
}