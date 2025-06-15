pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

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
