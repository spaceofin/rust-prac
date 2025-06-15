struct Circle {
    radius: f64,
}

fn main() {
    println!("Hello, world!");
    utils::greet("world");

    let target_number = 2;
    let square_result = utils::square(target_number);
    println!("The square of {target_number} is {square_result}.");

    let circle_a = Circle { radius: 3.5};
    let circle_b = Circle { radius: 5.0 };
    let area_a = utils::calculate_area(circle_a.radius);
    let area_b = utils::calculate_area(circle_b.radius);
    println!("The Area of circle a is {area_a}.");
    println!("The Area of circle b is {area_b}.");
}
