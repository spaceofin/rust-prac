use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(&self);
    fn print(&self);
}

impl<T> PrintInOption for T where
    Option<T>: Debug,
    T: Debug {
        fn print_in_option(&self) {
            println!("{:?}",Some(&self));
        }
        fn print(&self) {
            println!("{:?}",&self);
        }
}

#[derive(Debug)]
struct Years(i64);
struct Days(i64);

impl Years {
    // tuncates partial years
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool {
    age.0 >= 18
}

fn where_example() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
    vec.print();
}

fn new_type_example() {
    let age = Years(25);
    let age_days = age.to_days();
    println!("Is an adult? {}", is_adult(&age));
    println!("Is an adult? {}", is_adult(&age_days.to_years()));
    // println!("Is an adult? {}", is_adult(&age_days)); // Compile Error: expected '&Years', not '&Days'.

    let days = Days(10000);
    let days_age = days.to_years();
    println!("Your Age: {:?}", &days_age);
    println!("Is an adult? {}", is_adult(&days_age));

    let years = Years(42);
    let years_as_primitive_1: i64 = years.0; // Tuple
    let Years(years_as_primitive_2) = years; // Destructuring
    println!("years: {:?}", &years);
    println!("years as primitive 1: {}", years_as_primitive_1);
    println!("years as primitive 2: {}", years_as_primitive_2);
}

pub fn where_and_newtype_demo() {
    // where_example();
    new_type_example();
}