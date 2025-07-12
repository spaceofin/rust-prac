#![allow(dead_code)]

mod sample_mod1 {
    mod sub1 {
        pub fn print_msg() {
            println!("sample_mod1_sub1");
        }
    }
    pub mod sub2 {
        pub fn print_msg() {
            println!("sample_mod1_sub2");
        }
    }
    pub mod sub3 {
        pub fn print_msg1() {
            println!("sample_mod1_sub3_msg1");
        }
        pub fn print_msg2() {
            println!("sample_mod1_sub3_msg2");
        }
        pub fn print_msg3() {
            println!("sample_mod1_sub3_msg3");
        }
         pub fn print_msg4() {
            println!("sample_mod1_sub3_msg4");
        }
    }
}

pub mod sample_mod2 {
    mod sub1 {
        pub fn print_msg() {
            println!("sample_mod2_sub2");
        }
    }
    pub mod sub2 {
        pub fn print_msg() {
            println!("sample_mod2_sub2");
        }
    }
}

fn print_file_mods() {

    use sample_mod1::sub3::print_msg1;
    use sample_mod1::sub3::{print_msg2, print_msg3};
    use crate::mod_samples::runner::sample_mod1::sub3::print_msg4;

    // compile error: private module
    // sample_mod1::sub1::print_msg();
    
    sample_mod1::sub2::print_msg();

    // compile error: private module
    // sample_mod2::sub1::print_msg();
    
    sample_mod2::sub2::print_msg();

    print_msg1();
    print_msg2();
    print_msg3();
    print_msg4();
}

fn print_mod_samples() {
    // compile error: cannot declare a non-inline module inside a block
    // mod_basic;

    #[path = "mod_basic.rs"]
    mod mod_basic;

    use super::{mod_nested};

    mod_basic::print_msg();

    use mod_nested::nested_mod2::print_msg as print_nested_msg;

    print_nested_msg();
    mod_nested::nested_mod2::print_msg()
}


pub fn run() {
    println!("----------Modules in current file----------");
    print_file_mods();

    println!();
    println!("-------Modules in mod_samples module-------");
    print_mod_samples();
}