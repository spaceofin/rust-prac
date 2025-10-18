

struct CSStudent {
    name: &'static str,
    university: &'static str,
    fav_language: &'static str,
    git_username: &'static str,
}

trait Person {
    fn name(&self) -> &str;
}

impl Person for CSStudent {
    fn name(&self) -> &str {
        &self.name
    }
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn print_name(&self);
    fn university(&self) -> &str;
}

impl Student for CSStudent {
    fn print_name(&self) -> () {
        println!("name: {}", self.name());
    }
    fn university(&self) -> &str {
        &self.university
    }
}

trait Programmer {
    fn fav_language(&self) -> &str;
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> &str {
        &self.fav_language
    }
}

// CompSciStudent (computer science student) is a subtrait of both Programmer and Student.
// Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn print_university(&self);
    fn print_fav_language(&self);
    fn git_username(&self) -> &str;
}

impl CompSciStudent for CSStudent{
    fn print_university(&self) -> () {
        println!("university: {}", self.university());
    }
    fn print_fav_language(&self) -> () {
        println!("fav_language: {}", self.fav_language());
    }
    fn git_username(&self) -> &str {
        &self.git_username
    }
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn print_person_name(p: &dyn Person) {
    println!("person name: {}", p.name());
}

fn supertraits_examples() {
    let alice = CSStudent {
        name: "Alice",
        university: "Rust University",
        fav_language: "Rust",
        git_username: "alice123",
    };

    let alice_greeting = comp_sci_student_greeting(&alice);
    println!("{}", alice_greeting);

    alice.print_name();
    print_person_name(&alice);
}

pub fn supertraits_demo() {
    supertraits_examples();
}