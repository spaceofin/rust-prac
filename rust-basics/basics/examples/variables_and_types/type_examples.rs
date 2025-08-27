fn casting() {
    let decimal = 65.4321_f32;

    // compile error: Rust provides no implicit type conversion
    // let integer: u8 = decimal;
    
    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // let character = decimal as char;
    // compile error: invalid cast. a float cannot be directly converted to a char.

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when the value fits within the target type’s range
    println!("\nThe maximum value of u16 is: {}",u16::MAX);
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("\n1000 as a u8 is : {}", 1000_i32 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

     // when the value fits within the target type’s range
    println!("\nThe maximum value of i16 is: {}", i16::MAX);
    println!(" 128 as a i16 is: {}", 128 as i16);

    // In boundary case 128 value in 8-bit two's complement representation is -128
    println!("\nThe maximum value of i8 is: {}", i8::MAX);
    println!(" 128 as a i8 is : {}", 128_i32 as i8);

    // the value of 232 in 8-bit two's complement representation is -24
    println!(" 232 as a i8 is : {}", 232_i32 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    println!("\nfloat cast cases");
    println!("The maximum value of u8 is: {}", u8::MAX);
    println!("The minimum value of u8 is: {}", u8::MIN);
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        println!("\nfloat cast cases in unsafe block");
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

fn literals() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    // default type: i32
    let i = 1; 
    // default type: f64
    let f = 1.0; 
    let a = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // The type and size of `a` are determined by context.
    println!("size of `a` in bytes: {}", std::mem::size_of_val(&a));
    let b = a + 2f32;
    println!("size of `b` in bytes: {}", std::mem::size_of_val(&b));
}

fn inference() {
    let elem = 5u8;

    // empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point, the compiler just knows that it's a vector of something (`Vec<_>`).

    // Insert
    vec.push(elem);
    // Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)

    println!("{:?}", vec);
}

// `NanoSecond`, `Inch`, and `U64` are new names for `u64`.
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn aliasing() {
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);

    // Note that type aliases *don't* provide any extra type safety, because aliases are *not* new types
    // println!("{}",nanoseconds + 'a');
    // compile error
}

pub fn type_demo() {
    casting();
    literals();
    inference();
    aliasing();
}