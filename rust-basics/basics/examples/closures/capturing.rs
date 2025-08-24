fn closure_basic() {
    let outer_var = 55;

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i| { i + outer_var };
    // let closure_inferred  = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));

    let one = || 1;
    println!("closure returning one: {}", one());
}

pub fn capturing_demo() {
    // closure_basic()
    use std::mem;

    let color = String::from("green");
    // borrow color and stores the borrow and closure in the `print` variable.
    let print = || println!("`color`: {}", color);
    print();

    let _reborrow = &color;
    print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    inc();
    println!("`count after increment operations called`: {}", count);

    let _count_reborrowed = &mut count;
    println!("`count after reborrowed`: {}", count);

    let movable = Box::new(3);
    let always_copiable = 3;

    // `mem::drop` requires `T` so this must take by value.
    // A non-copy must move and so `movable` immediately moves into the closure.
    // auto capture
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    let non_consume = || {
        println!("`always_copiable`: {:?}", always_copiable);
        #[allow(dropping_copy_types)]
        mem::drop(always_copiable);
    };

    consume();
    // consume(); // compile error
    // println!("`movable`: {:?}", movable); // compile error

    non_consume();
    non_consume();
    println!("`always_copiable`: {:?}", always_copiable);

    let haystack = vec![1, 2, 3];
    println!("There're {} elements in haystack vec", haystack.len());
    println!("haystack contains 3: {}", haystack.contains(&3));

    let contains = move |needle| haystack.contains(needle);

    println!("haystack contains 1: {}", contains(&1));
    println!("haystack contains 4: {}", contains(&4));
    // println!("haystack contains 3: {}", haystack.contains(&3)); // compile error: value borrowed here after move

    let numbers = vec![4, 5, 6];
    println!("There're {} elements in numbers vec", numbers.len());
    println!("numbers contains 5: {}", numbers.contains(&5));

    let numbers_contains = |n| numbers.contains(n);

    println!("numbers contains 4: {}", numbers_contains(&4));
    println!("numbers contains 7: {}", numbers_contains(&7));
    println!("numbers contains 6: {}", numbers.contains(&6));
}