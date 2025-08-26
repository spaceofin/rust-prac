fn shadowing() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);
        // This binding *shadows* the outer one
        let shadowed_binding = "abc";
        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = "def";
    println!("shadowed in outer block: {}", shadowed_binding);

    // The previous shadowed_binding variable becomes inaccessible after shadowing,
    // and it will be dropped automatically when its scope ends.
}

fn declare_first() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);
    let another_binding;
    // println!("another binding: {}", another_binding);
    // compile error: variable `another_binding` is uninitialized

    another_binding = 1;
    println!("another binding: {}", another_binding);
}

fn freezing() {
    let mut mutable_integer = 7i32;

    {
        // Shadowing
        let mutable_integer = mutable_integer;
        println!("mutable_integer in inner block: {}",mutable_integer);

        // mutable_integer = 50;
        // compile error: `mutable_integer` is frozen in this scope

    }
    // `mutable_integer` is not frozen in this scope
    mutable_integer = 3; 
    println!("mutable_integer in outer block: {}",mutable_integer);
}

pub fn variable_bindings_demo() {
    shadowing();
    declare_first();
    freezing();
}