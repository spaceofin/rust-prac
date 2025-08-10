#[cfg(panic = "unwind")]
fn ah() {
    println!("Oops! It's not for you!!!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This place isn’t for you. Run!!!!");
}

fn drink_beverage(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!!!!");
        } else {
            println!("Spit it out!!!!");
        }
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

fn main() {
    drink("water");
    drink("lemonade");
    drink_beverage("soda");
    drink_beverage("lemonade");
}