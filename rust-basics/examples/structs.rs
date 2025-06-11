#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect:Rectangle) -> f32 {
    return (rect.bottom_right.x - rect.top_left.x) * (rect.top_left.y - rect.bottom_right.y);
}

fn square(point:Point, side: f32) -> Rectangle {
    return Rectangle {
        top_left: point.clone(),
        bottom_right: Point {x: point.x + side, y: point.y - side },
    };

}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 10.3, ..another_point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;

    let rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    
    let rectangle_area = rect_area(rectangle);
    println!("Rectangle area is {}", rectangle_area);

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let square_point: Point = Point { x: 0.0, y: 0.0 };
    let square = square(square_point, 5.0);
    println!("Square with start point (0,0) and side length 5.0 is {:?}", square)
}