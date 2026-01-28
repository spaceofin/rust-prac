fn raw_pointers() {
    let mut num = 5;
    
    // Using raw pointers allows having both an immutable and a mutable pointer 
    // to the same value at the same time (bypassing Rust's borrow checker).
    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn unsafe_code() {
    unsafe fn dangerous() {}
    
    // Compile Error: Calling an unsafe function requires an unsafe block
    // dangerous();

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}


pub fn run() {
    // raw_pointers();
    unsafe_code();
}