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


pub fn iteration_demo() {
    counter_demo();
}