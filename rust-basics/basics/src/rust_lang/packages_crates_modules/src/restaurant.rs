mod front_of_house;
mod back_of_house;

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