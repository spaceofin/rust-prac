#![allow(dead_code)]

pub mod commands;
pub mod testing{
    pub mod documentation_testing;
}

pub mod adder{
    pub fn add(a: i32, b:i32) ->  i32 {
        a + b
    }
}

pub mod multiplier{
    pub fn multiply(a: i32, b:i32) -> i32 {
        a * b
    }
}

pub mod library;