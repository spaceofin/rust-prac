#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

fn string_lifetime() {
    {
        let s: &'static str = "hello"; // s is valid from this point forward
        // s allocated on the stack and holds a reference to the string "hello" in read-only memory(not own).
    } // this scope is now over, and s is no longer valid

    // "hello" still alive.
    {
        let s = String::from("hello"); 
        // s allocated on the stack, owns the string "hello" on the heap
    } // s is dropped and the heap memory for "hello" is deallocated.
}

fn owned_string() {
    let s = String::from("hello");
    let mut s = String::from("helllo");
    s.push_str(", world");
    println!("{s}");
}

fn move_and_clone() {
    let x = 5;
    let y = x; // copy
    println!("x = {x}, y = {y}");

    let s1 = String::from("hello");
    let s2 = s1; // move;
    // Compile Error: s1 borrowed after move. s1 no longer valid.
    // println!("s1 = {s1}, s2 = {s2}");


    let mut s = String::from("hello");
    s = String::from("ahoy"); // s now points to a new String, previous heap data "hello" is deallocated.
    println!("{s}, world!");

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {s3}, s4 = {s4}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn ownership_examples() {
    let s = String::from("hello");
    takes_ownership(s);
    // Compile Error: s is no longer valid.
    // println!("s: {s}");

    let x = 5;
    makes_copy(x);
    println!("x: {x}");

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("s2: {s2}"); // Compile Error: borrowed after move.
    println!("s1: {s1}, s3: {s3}");

    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    // println!("s4: {s4}"); // Compile Error: borrowed after move.
    println!("The length of '{s5}' is {len}.");
}

fn main() {
    println!("\n----------Owned String----------");
    owned_string();
    println!("\n----------Move and Clone----------");
    move_and_clone();
    println!("\n----------Ownership Examples----------");
    ownership_examples();
}