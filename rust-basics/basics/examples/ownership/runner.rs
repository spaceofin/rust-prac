#![allow(dead_code)]

use super::borrowing;

pub fn run() {
    // println!("-------owned types-------");
    // ownership::owned_types::owned_types();
    // println!("-------copy types-------");
    // ownership::copy_types::copy_types();
    println!("--------borrowing-------");
    borrowing::borrowing();
}