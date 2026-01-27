use my_crate::PrimaryColor;
use my_crate::mix;
use rand;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let mixed_color = mix(red, yellow);
    println!("mixed color: {mixed_color:?}");

    let num = 10;
    println!("Hello, world! {num} plus one is {}!", adder::add_one(num));
    println!("Hello, world! {num} plus random number is {}!", adder::add_random(num));
    println!("Hello, world! {num} plus random number is {}!", num + (rand::random::<u8>() % 10) as i32);

}