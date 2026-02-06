fn raw_pointers() {
    let mut num = 5;
    
    // Using raw pointers allows having both an immutable and a mutable pointer 
    // to the same value at the same time (bypassing Rust's borrow checker).
    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn unsafe_code() {
    unsafe fn dangerous() {}
    
    // Compile Error: Calling an unsafe function requires an unsafe block
    // dangerous();

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

unsafe extern "C" {
    // functions known to be safe to call can be marked `safe`.
   safe fn abs(input: i32) -> i32;
}

fn extern_example() {
    // No `unsafe` block is needed because `abs` is marked as `safe`.
    // unsafe {
    //     println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

    println!("Absolute value of -3 according to C: {}", abs(-3));
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// In Rust, global variables are called static variables.
static HELLO_WORLD: &str = "Hello, world!";

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// static variables can be mutable.
static mut COUNTER: u32 = 0;

fn global_variables() {
    println!("value is: {HELLO_WORLD}");

    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        println!("COUNTER: {}", *(&raw const COUNTER));
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

trait Producer {
    type Output;

    fn produce(&self) -> Self::Output;
}


trait ProducerOf<T> {
    fn produce(&self) -> T;
}

#[derive(Debug)]
struct Counter;

impl Producer for Counter {
    type Output = u32;

    fn produce(&self) -> Self::Output {
        1
    }
}

impl ProducerOf<u32> for Counter {
    fn produce(&self) -> u32 {
        2
    }
}

impl ProducerOf<String> for Counter {
    fn produce(&self) -> String {
        "hello".to_string()
    }
}

fn associated_types_demo() {
    let counter = Counter;
    let product1 = <Counter as Producer>::produce(&counter);
    let product2 = <Counter as ProducerOf<u32>>::produce(&counter);
    let product3 = <Counter as ProducerOf<String>>::produce(&counter);

    println!("product1: {product1:?}");
    println!("product2: {product2:?}");
    println!("product3: {product3:?}");
}

use std::ops::Add;

// Default type parameter: `Rhs = Self` provides a default right-hand-side type, avoiding extra generic boilerplate in most cases.
// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // associated type
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// newtype pattern
#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

// Overrides the default type parameter (`Rhs = Self`) to add `Meters` to `Millimeters`.
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add for Meters {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

// <T = String>: default generic parameter
trait Greet<T = String> {
    fn greet(&self) -> T;
}

struct HelloBot {}

impl Greet for HelloBot {
    fn greet(&self) -> String {
        "hello".to_string()
    }
}

fn operator_overloading() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(
        Millimeters(1000) + Meters(2),
        Millimeters(3000)
    );

    assert_eq!(
        Meters(1) + Meters(2),
        Meters(3)
    );

    let hello_bot = HelloBot {};
    println!("HelloBot say: {}", hello_bot.greet());

    // Compile Error: `Add<Millimeters> for Millimeters` is not implemented
    // let _ = Millimeters(1000) + Millimeters(2000);
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;
struct Cat;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

impl Animal for Cat {
    fn baby_name() -> String {
        String::from("kitten")
    }
}


fn identically_named_methods() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());
    // Compile Error:
    // println!("A baby dog is called a {}", Animal::baby_name());
    
    // Disambiguating between identically named methods
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    println!("A baby cat is called a {}", <Cat as Animal>::baby_name());
}

use std::fmt;

// Display is a supertrait of OutlinePrint, so any type implementing OutlinePrint must also implement Display.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
struct Coordinates {
    x: i32,
    y: i32,
}

impl OutlinePrint for Coordinates {}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn supertraits() {
    let coord = Coordinates { x: 1, y: 2 };
    println!("Coorinate: {coord}");
    coord.outline_print();
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

use std::ops::Deref;

impl Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn newtype_pattern() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
    println!("len = {}", w.len());
    println!("is_empty = {}", w.is_empty());

    for s in w.iter() {
        println!("{s}");
    }
}

fn synonyms() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        f()
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hello"))
    }

    takes_long_type(f);
    let thunk = returns_long_type();
    thunk();
}

type Result<T> = std::result::Result<T, std::io::Error>;

trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}

#[derive(Debug)]
struct BufferWriter {
    buffer: Vec<u8>,
}

impl BufferWriter {
    fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }
}

impl Write for BufferWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

fn type_aliases() -> Result<()> {
    let mut writer = BufferWriter::new();
    let len1 = writer.write(b"hello ")?;
    let len2 = writer.write(b"world")?;
    println!("len1: {len1}, len2: {len2}");

    println!("writer: {writer:?}");
    writer.flush()?;

    println!("writer: {writer:?}");

    Ok(())
}

// Compile Error: expected `!`, found `()`.
// fn bar() -> ! {
//     let num = 10;
// }

// This function never returns;
// its return type `!` is the never type.
fn crash() -> ! {
    panic!("error");
}

fn never_type() {
    let nums = vec!["10", "hello", "20"];

    for s in nums {
        let n: u32 = match s.parse() {
            Ok(num) => num,
            Err(_) => continue, // `continue` is of the never type `!`
        };
        print!("{n} ");
    }
    println!();

    let vals = vec![Some(10), Some(20), Some(30)];

    for v in vals {
        let val = match v {
            Some(x) => x,
            // `panic!` is of the never type `!`
            None => panic!("Expected a value, but found None"),
        };
        print!("{val} ");
    }
    println!();
}

// The generic type parameter `T` implicitly implements the `Sized` trait
fn generic_sized<T>(t: T) {
    println!("Called generic_sized with a value!");
}

// The generic type parameter `T` may be unsized when using the `?Sized` trait bound
fn generic_unsized<T: ?Sized>(t: &T) {
    println!("Called generic_unsized with a reference!");
}

fn dst() {
    // Compile Error: doesn't have a size known at compile-time.
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    let a = String::from("hi");
    let b = String::from("hello");
    let c= "world";
    
    generic_sized(a);
    generic_sized(c);
    generic_unsized(&b);
    generic_unsized(c);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn function_pointers() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list of strings: {list_of_strings:?}");

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    let list_of_statuses_c: Vec<Status> = (0u32..20).map(|v| Status::Value(v)).collect();
    println!("list of statuses: {list_of_statuses:?}");
    println!("list of statuses(closure): {list_of_statuses_c:?}");
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
    move |x| x + init
}

fn returns_boxed_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_boxed_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}


fn returning_closures() {
    let boxed_handlers: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(returns_closure()), Box::new(returns_initialized_closure(123))];

    // Compile Error: expected opaque type, found a different opaque type
    // let handlers = vec![returns_closure(), returns_initialized_closure(123)];

    let handlers = vec![returns_boxed_closure(), returns_boxed_initialized_closure(123)];
    
    print!("\nboxed handlers: ");
    for handler in boxed_handlers {
        let output = handler(5);
        print!("{output} ");
    }

    print!("\nhandlers: ");
    for handler in handlers {
        let output = handler(5);
        print!("{output} ");
    }
}


// Declarative macro
// #[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Macros must be defined before use.

fn declarative_macros() {
    let mut my_vec = my_vec![1, 2, 3];
    println!("my_vec: {my_vec:?}");
}

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn procedural_macros() {
    Pancakes::hello_macro();
}


pub fn run() {
    // raw_pointers();
    // unsafe_code();
    // extern_example();
    // call_from_c();
    // global_variables();
    // associated_types_demo();
    // operator_overloading();
    // identically_named_methods();
    // supertraits();
    // newtype_pattern();
    // synonyms();
    // type_aliases().unwrap();
    // never_type();
    // dst();
    // function_pointers();
    // returning_closures();
    declarative_macros();
    procedural_macros();
}