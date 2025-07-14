
#[derive(Clone)]
struct Person {
  name: String,
  age: i32,
}

struct Persons {
  people: Vec<Person>,
}

impl Person {
  fn new(name: &str, age: i32) -> Self {
    Self { name: name.to_string(), age}
  }

  fn print_info(&self) {
    println!("Name: {}, Age: {}", self.name, self.age);
  }

  fn greet(&self) {
    println!("Hi, I'm {}!", self.name); 
  }
}

impl Persons {
    fn new() -> Self {
      // same as Self { people: Vec::new() }
      Persons { people: Vec::new() }
    }

    fn add(&mut self, person: Person) {
        self.people.push(person);
    }

    fn list(&self) {
        for p in &self.people {
            println!("{}({})", p.name, p.age);
        }
    }
}

pub fn print_persons() {
  let jimmy = Person::new("Jimmy",28);

  jimmy.print_info();
  jimmy.greet();

  let mut persons = Persons::new();

  let alex = Person::new("alex",35);
  let alice = Person::new("alice",23);

  persons.add(alex);
  persons.add(alice.clone());

  persons.list();

  // compile error: value borrowed here after move
  // alex.print_info();

  let charlie = Person {
    name: String::from("charlie"),
    ..alice
  };

  charlie.print_info();

}