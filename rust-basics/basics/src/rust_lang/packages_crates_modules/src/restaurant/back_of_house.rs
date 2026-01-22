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