
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
        #[allow(unreachable_code)]
        { println!("This point will never be reached"); }
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

fn fizz_buzz_while() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

fn fizz_buzz_for() {
    // for n in 1..=100 { ... } works the same
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn for_and_iterators() {
    let names = vec!["Bob", "Frank", "Ferris"];

    println!("----------iter----------");
    // iter: this borrows each element of the collection
    for name in names.iter() {
        match name { // &&str
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);

    println!("----------into_iter----------");
    // into_iter: this consumes the collection
    for name in names.into_iter() {
        match name { // &str
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    // compile error: the data has already been consumed by into_iter
    // println!("names: {:?}", names);

    println!("----------iter_mut----------");

    let mut names = vec!["Bob", "Frank", "Ferris"];
    // iter_mut: this mutably borrows each element of the collection
    for name in names.iter_mut() {
        *name = match name { // &mut &str
            &mut "Ferris" => "There is a rustacean among us!",
            &mut "Bob" => "Hello",
            _ => *name,
        }
    }

    println!("names: {:?}", names);
}

pub fn loops_demo() {
    // loops();
    // fizz_buzz_while();
    // fizz_buzz_for();
    for_and_iterators();
}