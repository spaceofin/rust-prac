struct Counter {
    count: usize,
    limit: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0, limit: usize::MAX }
    }
    fn new_with_limit(limit: usize) -> Counter {
        Counter { count: 0, limit }
    }
}

impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.limit {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn counter_demo() {
    let mut unlimited_counter = Counter::new();
    let mut limited_counter = Counter::new_with_limit(5);

    while let Some(n) = unlimited_counter.next() {
        println!("unlimited_counter: {}", n);
        if n == 20 { break; }
    }
    while let Some(n) = limited_counter.next() {
        println!("limited_counter: {}", n);
    }
}

fn iter_methods() {
    let vec = vec![1,2,3];
    let mut vec_iter = vec.iter();
    println!("vec.iter: {:?}",vec_iter);
    loop {
        let opt = vec_iter.next();
        println!("{:?}", opt);
        if opt.is_none() { break; }
    }
    println!("vec.iter after iteration: {:?}",vec_iter);
    println!("vec after iteration: {:?}",vec);
    println!();

    let mut vec_mut = vec![4,5,6];
    let mut vec_iter_mut = vec_mut.iter_mut();
    println!("vec.iter_mut: {:?}",vec_iter_mut);
    loop {
        let opt = vec_iter_mut.next();
        match opt {
            Some(x) => {
                *x *= 2;
                println!("{:?}", x);
            }
            None => break,
        }
    }
    println!("vec.iter_mut after iteration: {:?}",vec_iter_mut);
    println!("vec_mut after iteration: {:?}",vec_mut);
    println!();

    let vec2 = vec![7,8,9];
    let mut vec2_iter = vec2.into_iter();
    println!("arr.into_iter: {:?}",vec2_iter);
    loop {
        let opt = vec2_iter.next();
        println!("{:?}", opt);
        if opt.is_none() { break; }
    }
    println!("vec2.into_iter after iteration: {:?}",vec2_iter);
    // println!("vec2 after iteration: {:?}",vec2); // Compile error: ownership moved by into_iter().
    println!();

    // IntoIterator::into_iter([T; N]) calls the into_iter method
    // implemented by the array type.
    match IntoIterator::into_iter([10,11,12]) {
        mut iter => loop {
            let next;
            match iter.next() {
                Some(val) => next = val,
                None => break,
            };
            let x = next;
            println!("{x}");
        },
    };
    println!();
}


pub fn iteration_demo() {
    // counter_demo();
    iter_methods();
}