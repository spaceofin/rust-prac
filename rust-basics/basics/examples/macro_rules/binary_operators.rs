use std::ops::{Add, Sub, Mul, Div};

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBarAdd;
#[derive(Debug)]
struct FooBarSub;
#[derive(Debug)]
struct FooBarMul;
#[derive(Debug)]
struct FooBarDiv;

#[derive(Debug)]
struct BarFooAdd;
#[derive(Debug)]
struct BarFooSub;
#[derive(Debug)]
struct BarFooMul;
#[derive(Debug)]
struct BarFooDiv;

macro_rules! impl_binop {
    ($Trait:ident, $method:ident, $Lhs:ty, $Rhs:ty, $Out:ident) => {
        impl std::ops::$Trait<$Rhs> for $Lhs {
            type Output = $Out;

            fn $method(self, _rhs: $Rhs) -> $Out {
                println!("> {}.{}({}) was called", 
                    stringify!($Lhs),
                    stringify!($method),
                    stringify!($Rhs)
                );
                $Out
            }
        }
    };
}

impl_binop!(Add, add, Foo, Bar, FooBarAdd);
impl_binop!(Sub, sub, Foo, Bar, FooBarSub);
impl_binop!(Mul, mul, Foo, Bar, FooBarMul);
impl_binop!(Div, div, Foo, Bar, FooBarDiv);

impl_binop!(Add, add, Bar, Foo, BarFooAdd);
impl_binop!(Sub, sub, Bar, Foo, BarFooSub);
impl_binop!(Mul, mul, Bar, Foo, BarFooMul);
impl_binop!(Div, div, Bar, Foo, BarFooDiv);

pub fn test_binary_operators() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Foo - Bar = {:?}", Foo - Bar);
    println!("Foo * Bar = {:?}", Foo * Bar);
    println!("Foo / Bar = {:?}", Foo / Bar);
    
    println!("Bar + Foo = {:?}", Bar + Foo);
    println!("Bar - Foo = {:?}", Bar - Foo);
    println!("Bar * Foo = {:?}", Bar * Foo);
    println!("Bar / Foo = {:?}", Bar / Foo);
}