use std::cell::Cell;

fn immutable_reference_prevent_mutate() {
    let mut a = 10;
    let b = &a; // `b` is an immutable reference to `a`.
    // a = 20; // Compile Error: `a` was already borrowed.
    println!("a: {a}, b: {b}"); 
    // The immutable borrow ends here (`b` is no longer used).
    a = 20;
    println!("a: {a}");
}

fn cell_basic() {
    let a = Cell::new(10);
    let b = &a; // immutable reference to Cell
    println!("a: {a:?}, b: {b:?}"); 
    a.set(20);
    println!("a: {a:?}, b: {b:?}");
    b.set(30);
    println!("a: {}, b: {}", a.get(), b.get());
}

pub fn run() {
    // immutable_reference_prevent_mutate();
    cell_basic();
}