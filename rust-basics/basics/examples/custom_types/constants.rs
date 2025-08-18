static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;
static mut COUNTER: i32 = 0;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn increment_counter_and_print() {
    // static mut variable can only be accessed inside an unsafe block
    unsafe {
        COUNTER += 1;
        let counter = COUNTER;
        println!("COUNTER: {}", counter);
    }
}

pub fn constants_demo() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line

    increment_counter_and_print();
    increment_counter_and_print();
}