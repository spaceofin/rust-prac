#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);


// A form with both a UsernameWidget and an AgeWidget
struct Form {
    username: String,
    age: u8,
}

impl Form {
    fn get(&self) -> (String, u8) {
        (self.username.clone(), self.age)
    }
}

trait UsernameWidget {
    // Get the selected username out of this widget
    fn get(&self) -> String;
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

trait AgeWidget {
    // Get the selected age out of this widget
    fn get(&self) -> u8;
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn clone_examples() {
    // Instantiate `Unit`
    let unit = Unit;
    // Copy `Unit`, there are no resources to move
    let copied_unit = unit;

    // Both `Unit`s can be used independently
    println!("unit original: {:?}", unit);
    println!("unit copy: {:?}", copied_unit);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("pair original: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("pair moved: {:?}", moved_pair);

    //Compile Error: value borrowed after move
    //println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    println!("pair moved: {:?}", moved_pair);
    println!("pair clone: {:?}", cloned_pair);

    drop(moved_pair);
    // Compile Error: `moved_pair` has been dropped
    // println!("pair moved: {:?}", moved_pair);
    println!("pair clone: {:?}", cloned_pair);

}

fn overlapping_demo() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };
    // Compile Error: multiple `get` found.
    // println!("{}", form.get());

    // let (form_username, form_age) = <Form>::get(&form);
    // let (form_username, form_age) = Form::get(&form);
    let (form_username, form_age) = form.get();
    println!("username: {}, age: {}", form_username, form_age);

    // let username = <Form as UsernameWidget>::get(&form);
    let username = UsernameWidget::get(&form);
    println!("username: {}", username);

    // let age = <Form as AgeWidget>::get(&form);
    let age = AgeWidget::get(&form);
    println!("age: {}", age);
}

pub fn clone_and_overlapping_demo() {
    // clone_examples();
    overlapping_demo();
}