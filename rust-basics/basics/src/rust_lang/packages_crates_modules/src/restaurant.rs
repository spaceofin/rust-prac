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

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        pub fn seasonal_fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    // Making an enum public makes all of its variants public as well.
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn deliver_order() {}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::restaurant::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // Public function
    hosting::add_to_waitlist();

    // Compile Error: Private function is inaccessible
    // hosting::take_order();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    println!("I'd like {} toast with {} please", meal.toast, meal.seasonal_fruit());

    meal.toast = String::from("Wheat");
    // Compile error: `seasonal_fruit` is a private field and cannot be modified
    // meal.seasonal_fruit = String::from("Banana");
    println!("I'd like {} toast with {} please", meal.toast, meal.seasonal_fruit());

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("order1: {order1:?}");
    println!("order2: {order2:?}");
}