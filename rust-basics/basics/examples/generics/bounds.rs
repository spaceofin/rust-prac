// A trait which implements the print marker: `{:?}`.
use std::fmt::Debug;

trait CanSpeak {
    fn speak(&self) -> &'static str;
}

#[derive(Debug)]
struct Dog;
struct Cat;

impl CanSpeak for Dog {
    fn speak(&self) -> &'static str { 
        "bark!" 
    }
 }

impl CanSpeak for Cat {
    fn speak(&self) -> &'static str { 
        "meow~"
    }
}

fn make_speak<T: CanSpeak>(animal: &T) -> &'static str {
    animal.speak()
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn animal_demos() {
    let dog = Dog;
    let cat = Cat;


    print_debug(&dog);
    // Compile error: Cat does not implement the Debug trait
    // print_debug(&cat);

    println!("{}", make_speak(&dog));
    println!("{}", make_speak(&cat));
}

struct RedType;
struct BlueType;
struct UnknownType;

trait Red {}
trait Blue {}

impl Red for RedType {}
impl Blue for BlueType {}

fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn color_demos() {
    // empty bounds example

    let red_type = RedType;
    let blue_type = BlueType;
    let unknown_type = UnknownType;

    println!("RedType is {}", red(&red_type));
    println!("BlueType is {}", blue(&blue_type));
    // Compile Error: the trait `Red` is not implemented for `UnknownType`
    // println!("UnknownType is {}", red(&unknown_type));
}

pub fn bounds_examples() {
    animal_demos();
    color_demos();

}