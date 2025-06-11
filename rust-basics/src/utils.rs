use std::f64::consts::PI;

pub fn greet(name: &str) {
    println!("Hello, {name}!");
}

pub fn square(x: u32) -> u32 { 
  return x * x;
}

pub fn calculate_area(radius: f64) -> f64 {
  return PI * radius * radius;
}
