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

unsafe extern "C" {
    // functions known to be safe to call can be marked `safe`.
   safe fn abs(input: i32) -> i32;
}

fn extern_example() {
    // No `unsafe` block is needed because `abs` is marked as `safe`.
    // unsafe {
    //     println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

    println!("Absolute value of -3 according to C: {}", abs(-3));
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// In Rust, global variables are called static variables.
static HELLO_WORLD: &str = "Hello, world!";

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// static variables can be mutable.
static mut COUNTER: u32 = 0;

fn global_variables() {
    println!("value is: {HELLO_WORLD}");

    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        println!("COUNTER: {}", *(&raw const COUNTER));
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

trait Producer {
    type Output;

    fn produce(&self) -> Self::Output;
}


trait ProducerOf<T> {
    fn produce(&self) -> T;
}

#[derive(Debug)]
struct Counter;

impl Producer for Counter {
    type Output = u32;

    fn produce(&self) -> Self::Output {
        1
    }
}

impl ProducerOf<u32> for Counter {
    fn produce(&self) -> u32 {
        2
    }
}

impl ProducerOf<String> for Counter {
    fn produce(&self) -> String {
        "hello".to_string()
    }
}

fn associated_types_demo() {
    let counter = Counter;
    let product1 = <Counter as Producer>::produce(&counter);
    let product2 = <Counter as ProducerOf<u32>>::produce(&counter);
    let product3 = <Counter as ProducerOf<String>>::produce(&counter);

    println!("product1: {product1:?}");
    println!("product2: {product2:?}");
    println!("product3: {product3:?}");
}

pub fn run() {
    // raw_pointers();
    // unsafe_code();
    // extern_example();
    // call_from_c();
    // global_variables();
    associated_types_demo();
}