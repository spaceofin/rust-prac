#![allow(dead_code)]

mod ownership;

fn main() {
    // println!("-------owned types-------");
    // ownership::owned_types::owned_types();
    // println!("-------copy types-------");
    // ownership::copy_types::copy_types();
    println!("--------borrowing-------");
    ownership::borrowing::borrowing();
}