// A trait which implements the print marker: `{:?}`.
use std::fmt::{Debug, Display, Formatter, self};

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

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("Debug t: {:?}", t);
    println!("Debug u: {:?}", u);
}

fn compare_formats<T: Debug + Display, U: Debug + Display>(t: &T, u:&U) {
    println!("Debug t: {:?}", t);
    println!("Debug u: {:?}", u);
    println!("Display t: {}", t);
    println!("Display u: {}", u);
}

#[derive(Debug)]
struct MyArray<T, const N: usize> ([T; N]);

#[derive(Debug)]
struct MyVec<T> (Vec<T>);

impl<T, const N: usize> MyArray<T, N> {
    fn new(data: [T; N]) -> Self {
        Self(data)
    }
}

impl<T> MyVec<T> {
    fn new(data: Vec<T>) -> Self {
        Self(data)
    }
}

impl<T: Display, const N: usize> Display for MyArray<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.0.iter().enumerate() {
            write!(f, "{}", item)?;
            if i < N - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T: Display> Display for MyVec<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.0.iter().enumerate() {
            write!(f, "{}", item)?;
            if i < self.0.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

fn multiple_bounds_demo() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // Compile error: doesn't implement `std::fmt::Display`
    // compare_prints(&array);

    compare_types(&array, &vec);

    let my_array = MyArray::new([4, 5, 6]);
    let my_vec = MyVec::new(vec![4, 5, 6]);

    compare_formats(&my_array, &my_vec);
}

pub fn bounds_examples() {
    animal_demos();
    color_demos();
    multiple_bounds_demo();
}