use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `T` is bounded such that any *references* in `T` must outlive `'a`
// Additionally, the lifetime of `Ref` may not exceed `'a`.

#[derive(Debug)]
struct Ref2<'a, 'b, T: 'a, U: 'b>(&'a T, &'b U);

fn print<T>(t: T) where
    T: Debug {
        println!("`print`: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

// Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long a `'b`.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn bounds_demo() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);

    let y = 8;
    let ref_x_y = Ref2(&x, &y);
    print_ref(&ref_x_y);
    print(ref_x_y);
}

fn coercion_demo() {
    let first = 2; // longer lifetime
    {
        let second = 3; // shorter lifetime
    
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}

pub fn bounds_and_coercion_demo() {
    // bounds_demo();
    coercion_demo();
}