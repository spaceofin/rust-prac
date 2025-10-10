

#[derive(Debug, Clone)] enum Food { Apple, Carrot, Potato, CordonBleu, Steak, Sushi }

#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None       => None,
    }
}


fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None               => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// We chain multiple uses of `map()` to simplify the code.
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// Check whether there's food or not before trying to eat it!
fn eat_cooked(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None       => println!("Oh no! It wasn't edible."),
    }
}

// We don't have the ingredients to make Sushi.
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

// We have the recipe for everything except Cordon Bleu.
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

// To make a dish, we need both the recipe and the ingredients.
// We can represent the logic with a chain of `match`es:
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None       => None,
        Some(food) => have_ingredients(food),
    }
}

// This can conveniently be rewritten more compactly with `and_then()`:
fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

// Otherwise we'd need to `flatten()` an `Option<Option<Food>>`
// to get an `Option<Food>`:
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingredients).flatten()
}

fn eat(food: Food, day: Day) {
    print!("[Testing cookable_v1] ");
    match cookable_v1(food.clone()) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }

    print!("[Testing cookable_v2] ");
    match cookable_v2(food.clone()) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }

    print!("[Testing cookable_v3] ");
    match cookable_v3(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn cook_and_eat_demo() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    
    // Let's try the simpler looking `process()` now.
    let cooked_potato = process(potato);

    eat_cooked(cooked_apple);
    eat_cooked(cooked_carrot);
    eat_cooked(cooked_potato);

    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}

#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon, Grape, Mango, Cherry }

pub fn fruit_demo() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let mut no_fruit: Option<Fruit> = None;

    println!("__________or__________");
    let first_available_fruit_1 = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}", first_available_fruit_1);
    // println!("Variable apple was moved, so this line won't compile: {:?}", apple);

    println!("__________or_else__________");

    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };
    
    no_fruit = None;
    let first_available_fruit_2 = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}", first_available_fruit_2);

    println!("__________get_or_insert__________");
    let mut my_fruit: Option<Fruit> = None;
    let grape = Fruit::Grape;
    let first_available_fruit_3 = my_fruit.get_or_insert(grape);
    println!("first_available_fruit is: {:?}", first_available_fruit_3);
    println!("my_fruit is: {:?}", my_fruit);

    println!("__________get_or_insert_with__________");
    let get_mango_as_fallback = || {
        println!("Providing mango as fallback");
        Fruit::Mango
    };

    my_fruit = None;
    let first_available_fruit_4 = my_fruit
        .get_or_insert_with(get_mango_as_fallback);
    println!("first_available_fruit is: {:?}", first_available_fruit_4);
    println!("my_fruit is: {:?}", my_fruit);

    let mut my_cherry = Some(Fruit::Cherry);
    let should_be_cherry = my_cherry.get_or_insert_with(get_mango_as_fallback);
    println!("should_be_cherry is: {:?}", should_be_cherry);
    println!("my_cherry is unchanged: {:?}", my_cherry);

}

pub fn combinators_demo() {
    // cook_and_eat_demo();
    fruit_demo();
}