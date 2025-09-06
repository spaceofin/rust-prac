#![allow(dead_code)]

pub mod deeply {
    pub mod nested {
        pub fn my_first_function() {
            println!("my_first_function called.");
        }
        pub fn my_second_function() {
            println!("my_second_function called.")
        }
        pub fn my_third_function() {
            println!("my_third_function called.")
        }
        pub fn function() {
            println!("deeply::nested::function called.")
        }
        pub trait AndATraitType {}
    }
}


pub mod cool {
    pub fn function() {
        println!("cool::function called.");
    }
}


pub mod mod_basic;
pub mod mod_nested;
pub mod visibility;
pub mod mod_examples;
pub mod runner;