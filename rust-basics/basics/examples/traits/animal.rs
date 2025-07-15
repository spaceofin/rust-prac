trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn status(&self);

    fn talk(&self) {
        println! ("{} says {}", self.name(), self.noise());
    }
}

struct Cat { name: &'static str, asleep: bool }

impl Cat {
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

pub fn animal_demo() {
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