use std::any::type_name;
use std::fmt::{self, Debug};

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

#[derive(Debug)]
struct A;
struct Single(A);
struct SingleGen<T>(T); // Generic type 'SingleGen'

#[derive(Debug)]
struct S(A);
#[derive(Debug)]
struct SGen<T>(T); // Generic type 'SGen'

struct GenericVal<T>(T); // Generic type `GenericVal`

// impl of GenericVal where we explicitly specify type parameters:
impl GenericVal<f32> {} // Specify `f32`
impl GenericVal<S> {} // Specify `S` as defined above

// `<T>` Must precede the type to remain generic
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn reg_fn(s: S) {
    println!("{:?}",s);
}

fn gen_spec_t(s: SGen<A>) { 
    println!("{:?}",s);
}

fn gen_spec_i32(s: SGen<i32>) {
    println!("{:?}",s);
}

fn generic<T: std::fmt::Debug>(s: SGen<T>) {
    println!("{:?}",s);
}

// Non-copyable types.
#[derive(Debug)]
struct Empty;
#[derive(Debug)]
struct Null;

// A trait generic over `T`.
trait DoubleDrop<T> {
    fn double_drop(self, other: T);
}

impl<T,U> DoubleDrop<T> for U
where
    U: Debug,
    T: Debug,
{
    fn double_drop(self, other:T) {
        println!("Dropping {:?} and {:?}", self, other);
    }
}

fn generic_basic() {
    let _s = Single(A);
    let _char: SingleGen<char> = SingleGen('a');
    let _t    = SingleGen(A); 
    let _i32  = SingleGen(6); 
    let _char = SingleGen('a'); 

    print_type_of(&5);      // i32
    print_type_of(&'a');    // char
    print_type_of(&"asdf"); // &str
    print_type_of(&String::from("asdf")); // String

    print_type_of(&_s);     // Single
    print_type_of(&_char);  // SingleGen<char>
    print_type_of(&_t);     // SingleGen<A>
    print_type_of(&_i32);   // SingleGen<i32>
}

fn generic_functions() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

    // Using the generic functions
    generic::<char>(SGen('a')); // Explicitly specified type parameter `char` to `generic()`.
    generic(SGen('c')); // Implicitly specified type parameter `char` to `generic()`.
}

fn generic_impls() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    
    // Compile error: expected `f64`, found integer
    // let a = Val { val: 3};
    let b = GenVal { gen_val: 3 };
    let c = GenVal { gen_val: "hi" };

    println!("{}, {}", x.value(), y.value());
    println!("{}, {}", b.value(), c.value());
}

fn generic_traits() {
    let empty = Empty;
    let null = Null;

    // Deallocate 'empty' and 'null'.
    empty.double_drop(null);
}

pub fn generic_examples() {
    // generic_basic();
    // generic_functions();
    // generic_impls();
    generic_traits();
}