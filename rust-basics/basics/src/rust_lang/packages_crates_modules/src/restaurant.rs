mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist was called.");
        }

        pub fn seat_at_table() {
            println!("seat_at_table was called.");
        }
    }

    mod serving {
        fn take_order() {
            println!("take_order was called.");
        }

        fn serve_order() {
            println!("serve_order was called.");
        }

        fn take_payment() {
            println!("take_payment was called.");
        }
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::restaurant::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}