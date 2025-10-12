trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn status(&self) {
        println!("{} is resting", self.name());
    }

    fn talk(&self) {
        println! ("{} says {}", self.name(), self.noise());
    }
}

struct Cat { name: &'static str, asleep: bool }

impl Cat {
    fn new() -> Self {
        Cat { name: "Unknown", asleep: false }
    }

    fn is_asleep(&self) -> bool {
        self.asleep
    }

    fn nap(&mut self) {
        if self.is_asleep() {
            println!("{} is already asleep...", self.name());
        } else {
            println!("{} curls up and falls asleep.", self.name);
            self.asleep = true;
        }
    }

    fn wake_up(&mut self) {
        if self.is_asleep() {
            println!("{} wakes up!", self.name);
            self.asleep = false;
        } else {
            println!("{} is already awake.", self.name);
        }
    }
}

impl Animal for Cat {
    fn new(name: &'static str) -> Cat {
        Cat { name, asleep: false }
    }

    fn name(&self) -> &'static str { 
        self.name
    }
    
    fn noise(&self) -> &'static str {
        if self.is_asleep() {
            "meow... (yawn)"
        } else {
            "meow!"
        }
    }

    fn status(&self) {
        if self.asleep {
            println!("{} is asleep", self.name);
        } else {
            println!("{} is awake", self.name);
        }
    }

    fn talk(&self) {
        println! ("{} says {}", self.name(), self.noise());
    }
}

struct Sheep { name: &'static str }
struct Cow { name: &'static str }

impl Sheep {
    fn new() -> Self {
        Sheep { name: "Unknown" }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

impl Cow {
    fn new() -> Self {
        Cow { name: "Unknown" }
    }
}


impl Animal for Cow {
    fn new(name: &'static str) -> Self {
        Cow { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

fn cat_example() {
    let mut muffin: Cat = Animal::new("Muffin");

    muffin.status();
    muffin.talk();

    muffin.nap();
    muffin.status();
    muffin.talk();

    muffin.wake_up();
    muffin.status();
    muffin.talk();
}

fn animal_examples() {

    let simba: Cat = Animal::new("simba");
    let dolly: Sheep = Animal::new("dolly");
    let moomoo: Cow = Animal::new("moomoo");

    simba.talk();
    dolly.talk();
    moomoo.talk();

    let anonymous_cat = Cat::new();
    let anonymous_sheep = Sheep::new();
    let anonymous_cow = Cow::new();
    
    anonymous_cat.talk();
    anonymous_sheep.talk();
    anonymous_cow.talk();
}

pub fn animal_demo() {
    animal_examples();
}