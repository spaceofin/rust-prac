struct Cat {}
struct Sheep {}
struct Cow {}

trait AnonymousAnimal {
    fn noise(&self) -> &'static str;
}

impl AnonymousAnimal for Cat {
    fn noise(&self) -> &'static str {
        "meow!"
    }
}

impl AnonymousAnimal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

impl AnonymousAnimal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn AnonymousAnimal> {
    if random_number < 0.33 {
        Box::new(Cat {})
    } else if random_number < 0.66 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

use rand::Rng;

pub fn anonymous_animal_demo() {
    let mut rng = rand::thread_rng();
    let random_number:f64 = rng.r#gen();
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());

}