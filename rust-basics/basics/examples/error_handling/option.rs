use std::panic;

fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No drink? Oh well."),
    }
}

fn drink(drink: Option<&str>) {
    let result = panic::catch_unwind(|| {
        // `unwrap` returns a `panic` when it receives a `None`.
        let inside = drink.unwrap();
        if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

        println!("I love {}s!!!!!", inside);
    });

    match result {
        Ok(_) => {}
        Err(_) => println!("_____panic occurred, but the program continues running_____")
    }
}

fn drink_demo() {
    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(lemonade);
    drink(nothing);
}

fn next_age(name: &str, current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year {} will be {}", name, next_age))
}

#[derive(Debug)]
struct Person {
    name: Option<String>,
    job: Option<Job>,
    age: Option<u8>,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }

    fn next_age(&self) -> Option<String> {
        let next_age: u8 = self.age? + 1;
        Some(format!("Next year, {} will be {}", self.name.as_deref().unwrap_or("unknown"), next_age))
    }
}

fn print_person_info(person: &Person) {
    println!("\n__________ {} info __________",person.name.as_deref().unwrap_or("unknown"));
    println!("{:?}",person);
    println!("{}", person.next_age().unwrap_or("Age information not available".to_string()));
}

fn person_demo() {
    let alice = Person {
        name: Some("alice".to_string()),
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439111111,
            }),
        }),
        age: Some(27),
    };

    let bob = Person {
        name: Some("bob".to_string()),
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(51),
                number: 439222222,
            }),
        }),
        age: Some(25),
    };

    let anonymous = Person {
        name: None,
        job: None,
        age: None,
    };

    print_person_info(&alice);
    print_person_info(&bob);
    print_person_info(&anonymous);
}

pub fn option_demo() {
    // drink_demo();
    person_demo();
}