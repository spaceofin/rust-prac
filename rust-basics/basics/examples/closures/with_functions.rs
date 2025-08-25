
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    f(3)
}

fn function() {
    println!("I'm a function!");
}

fn closures_as_input() {
    use std::mem;

    // copy type
    let greeting: &'static str = "hello";
    println!("{}",greeting);
    println!("{}",greeting);

    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell: String = "goodbye".to_owned();

    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);
    // apply(diary); // compile error: value used here after move

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    let x = 7;
    let mut y = 7;
    let z = String::from("9");

    let print_x = || println!("x: {}", x);
    let print_y = || { 
        y += 1;
        println!("y: {}", y);
    };

    // `move` is omitted, but `drop(z)` forces the closure to capture `z` by value (FnOnce)
    let print_z = || {
        // `z` is passed as `&z` to `println!` (requires `Display`), so ownership is not moved
        println!("z: {}, and z will be dropped",z);
        drop(z);
    };

    apply(print_x);
    apply(print_y);
    apply(print_z);

    println!("Functions can also be used as arguments like closures.");
    apply(function);
}

// `move` makes the closure take ownership of `text`
// Without `move`, `text` would be captured by reference, 
// leading to a dangling reference when the function returns
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

fn closures_as_output() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}


pub fn with_functions_demo() {
    closures_as_input();
    closures_as_output();
}