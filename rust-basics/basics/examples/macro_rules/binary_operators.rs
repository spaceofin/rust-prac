use std::ops::{Add, Sub, Mul, Div};
use std::iter;

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


macro_rules! assert_equal_len {
    // expr: expression, ident: variable/function names, tt: operators/tokens
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!($a.len() == $b.len(),
                "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),));
    };
}

macro_rules! op {
    ($func:ident, $bound: ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}

// Implement `add_assign`, `mul_assign`, and `sub_assign` functions.
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

macro_rules! test {
    ($test_name:ident, $func:ident, $x:expr, $y:expr, $z:expr) => {
        fn $test_name() {
            for size in 0usize..10 {
                let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                let y: Vec<_> = iter::repeat($y).take(size).collect();
                let z: Vec<_> = iter::repeat($z).take(size).collect();

                $func(&mut x, &y);

                if x == z {
                    println!("[test {}] size {} pass", stringify!($func), size);
                } else {
                    println!("[test {}] size {} failed", stringify!($func), size);
                    println!("  actual:   {:?}", x);
                    println!("  expected: {:?}", z);
}
            }
        }
    };
}

pub fn test_binary_operators() {
    // println!("Foo + Bar = {:?}", Foo + Bar);
    // println!("Foo - Bar = {:?}", Foo - Bar);
    // println!("Foo * Bar = {:?}", Foo * Bar);
    // println!("Foo / Bar = {:?}", Foo / Bar);
    
    // println!("Bar + Foo = {:?}", Bar + Foo);
    // println!("Bar - Foo = {:?}", Bar - Foo);
    // println!("Bar * Foo = {:?}", Bar * Foo);
    // println!("Bar / Foo = {:?}", Bar / Foo);


    test!(test_add_assign, add_assign, 1u32, 2u32, 3u32);
    test!(test_add_assign_2, add_assign, 1u32, 2u32, 4u32);
    test!(test_mul_assign, mul_assign, 2u32, 3u32, 6u32);
    test!(test_sub_assign, sub_assign, 3u32, 2u32, 1u32);

    test_add_assign();
    test_add_assign_2();
    test_mul_assign();
    test_sub_assign();
}
