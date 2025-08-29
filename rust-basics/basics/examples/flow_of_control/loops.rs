fn loops (){
    println!("----------infinite loop----------");
    let mut count = 0u32;
    println!("Let's count until infinity!");
    // Infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }

    println!("\n----------outer and inner loop----------");
    'outer: loop {
        println!("Entered the outer loop");
        loop {
            println!("Entered the inner loop");
            //break; // This would break only the inner loop
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    println!("\n----------nesting and labels----------");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("counter: {}", counter);
    println!("result: {}", result);
}

pub fn loops_demo() {
    loops();
}