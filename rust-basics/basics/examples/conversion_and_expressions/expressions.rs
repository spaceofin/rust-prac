#[allow(path_statements)]
#[allow(unused_must_use)]
fn kinds_of_statements() {
    // variable binding is statements.
    let x = 5;

    // Expressions followed by a semicolon become statements
    x;
    x + 1;
    15;

    // blocks are expressions
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

pub fn expressions_demo() {
    kinds_of_statements();
}