struct Fibonacci {
    curr: u32,
    next: u32,
}

pub struct Take<I> {
    iter: I,
    n: usize,
}

impl<I> Take<I> {
    fn new(iter: I, n: usize) -> Take<I> {
        Take { iter, n }
    }
}

impl<I: Iterator> Iterator for Take<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 { return None; }
        self.n -= 1;
        self.iter.next()
    }
}

impl Fibonacci {
    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        println!("*** custom take({}) called ***",n);
        Take::new(self, n)
    }
}

impl Iterator for Fibonacci {
    // associated type
    // We can refer to this type using Self::Item
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = self.next + current;

        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn fibonacci_examples() {
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
   
}

pub fn iterators_demo() {
    fibonacci_examples();
}