// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
    // `c` is destroyed and the memory freed
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn ownership_examples() {
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a: Box<i32> = Box::new(5i32);
    println!("a contains: {}", a);
    println!("a contains: {}", &a);

    // *Move* `a` into `b`
    let b = a;
    // Compile Error: value borrowed after move
    // println!("a contains: {}", a);
    println!("b contains: {}", b);

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);
    // Compile Error: `b` has been moved
    // println!("b contains: {}", b);
}

fn partial_moves_examples() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    // store `age` variable on the heap to illustrate the partial move.
    // if `age` stored on the stack, `age` would copy the data without moving it.
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}

fn borrowing_examples() {

    // Create a boxed i32 in the heap, and a i32 on the stack
    // Remember: numbers can have arbitrary underscores added for readability
    // 5_i32 is the same as 5i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        // Rust automatically dereferences `Box<i32>` when a `&i32` is expected. `&boxed_i32` is effectively the same as `&*boxed_i32`.
        let ref_to_i32: &i32 = &*boxed_i32;

        println!("ref_to_i32: {}", ref_to_i32);

        // Compile Error:
        // Can't destroy `boxed_i32` while the inner value is borrowed late in scope.
        // destroy_box(boxed_i32);

        borrow_i32(ref_to_i32);
        // `ref_to_i32` goes out of scope and is no longer borrowed.
    }

    // `boxed_i32` can now give up ownership to `destroy_box` and be destroyed
    destroy_box(boxed_i32);
}

pub fn ownership_and_moves_demo() {
    // ownership_examples();
    // partial_moves_examples();
    borrowing_examples();
}