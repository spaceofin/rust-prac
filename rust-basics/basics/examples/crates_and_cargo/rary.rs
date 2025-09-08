pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

// To compile as a library (produces .rlib):
// rustc --crate-type=lib rary.rs

// Alternatively, specify a crate name (without .rlib):
// rustc --crate-type=lib --crate-name=rary rary.rs