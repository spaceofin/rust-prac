use std::{fmt,mem};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64, // 8bytes
    y: f64, // 8bytes
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point, // 16bytes
    bottom_right: Point, // 16bytes
}

fn origin() -> Point {
    Point { x:0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point { x:0.0, y:0.0 })
}

#[derive(Clone)]
struct MyBox<T>(Box<T>);

impl<T: fmt::Display> fmt::Display for MyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Box<{}>", self.0)
    }
}


pub fn box_examples() {
    // stack allocated variables
    let point = origin();
    let rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // Double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("point:{:?}",point);
    println!("rectangle:{:?}",rectangle);
    println!("boxed_rectangle:{:?}",boxed_rectangle);
    println!("boxed_point:{:?}",boxed_point);
    println!("box_in_a_box:{:?}",box_in_a_box);
    println!();

    println!("Point occupies {} bytes on the stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack", mem::size_of_val(&rectangle));
    // box size == pointer size
    // Stack: 8 bytes pointer (on a 64-bit system)
    println!("Boxed point occupies {} bytes on the stack", mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack", mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack", mem::size_of_val(&box_in_a_box));
    
    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("unboxed_point:{:?}",unboxed_point);
    println!("Unboxed point occupies {} bytes on the stack", mem::size_of_val(&unboxed_point));

    println!();
    let my_boxed_point: MyBox<Point> = MyBox(Box::new(origin()));
    let my_box_in_a_box: MyBox<MyBox<Point>> = MyBox(Box::new(my_boxed_point.clone()));
    println!("my_boxed_point:{}",my_boxed_point);
    println!("my_box_in_a_box:{}",my_box_in_a_box);
}