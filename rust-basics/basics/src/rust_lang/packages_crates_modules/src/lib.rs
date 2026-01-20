#![allow(dead_code)]

pub mod restaurant;

pub mod parent {
    fn parent_private() {
        println!("parent private");
    }

    pub fn parent_public() {
        println!("parent public");
    }

    // Private items are accessible within the module where they are defined and its submodules
    mod child {
        pub fn child_fn() {
            println!("__child_fn called__");
            crate::parent::parent_private();
            super::parent_private();
            super::parent_public();
            println!("__child_fn finished__");
        }

        mod subchild {
            pub fn sub_child_fn() {
                println!("__subchild_fn called__");
            }
        }
    }

    pub fn parent_fn() {
        child::child_fn(); // `child` is private, but visible within the same module
        // child::subchild::sub_child_fn(); // `subchild` is private and not accessible from its parent module
    }
}

pub mod public {
    pub fn public_fn() {
        println!("__public_fn called__");
    }
}

// re-export
pub use restaurant::eat_at_restaurant;
pub use parent::{parent_fn, parent_public};
pub use public::public_fn;

// Compile Error: Private items can't re-export
// pub use parent::{parent_private, child};