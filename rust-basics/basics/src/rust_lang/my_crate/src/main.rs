use my_crate::PrimaryColor;
use my_crate::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let mixed_color = mix(red, yellow);
    println!("mixed color: {mixed_color:?}");
}