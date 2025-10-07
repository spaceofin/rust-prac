fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 { return false; }

    // This is an expression, the 'return' keyword is not necessary here
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // associated function
    // This function is associated with a particular type, that is, Point.
    // Associated functions don't need to be called with an instance. These functions are generally used like constructors.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // associated function
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    fn describe() -> () {
        println!("This is Associated Function for Point.");
    }

    fn print(&self) -> () {
        println!("x: {:?}, y: {:?}",self.x, self.y);
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn describe() -> () {
        println!("This is Associated Function for Rectangle.");
    }

    // method
    // `&self` is sugar for `self::&Self`, where `Self` is the type of the caller object. In thie case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    // method
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method require the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}

fn functions_examples() {
    // fizzbuzz_to(20);

    // call describe associated function
    Point::describe();

    let point_a = Point {x: 1.0, y: 2.0};
    // call print method 
    point_a.print();

    // call describe associated function
    Rectangle::describe();

    let rectangle = Rectangle {
        // Associated functions are called using double colons(::)
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed when calling a method.
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    //pair.destroy();
}

pub fn functions_and_methods_demo() {
    functions_examples();
}