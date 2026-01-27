use rand::Rng;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_random(x: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_value: i32 = rng.gen_range(1..=10);
    x + random_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}