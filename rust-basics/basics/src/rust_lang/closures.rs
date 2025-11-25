use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn closures_demo() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn closures_types() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let result = expensive_closure(10);
    println!("caculation result: {}", result);

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;

    println!("add_one_result: {} {} {} {}",
        add_one_v1(1), add_one_v2(2), add_one_v3(3), add_one_v4(4));

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // Compile Error: the closure has been inferred to take a `String`,
    // let n = example_closure(5);
    println!("s: {s}");
}

fn closures_capturing_or_moving() {
    let im_list = vec![1, 2, 3];
    println!("[im_list] Before defining closure: {im_list:?}");

    // borrowing immutably
    let only_borrows = || println!("[im_list] From closure: {im_list:?}");

    println!("[im_list] Before calling closure: {im_list:?}");
    only_borrows();
    println!("[im_list] After calling closure: {im_list:?}");

    let mut mu_list = vec![4, 5, 6];
    println!("[mu_list] Before defining closure: {mu_list:?}");

    // borrowing mutably
    let mut borrows_mutably = || mu_list.push(7);

    // Compile Error: immutable borrow occurs.
    // println!("[mu_list] Before calling closure: {mu_list:?}");
    borrows_mutably();
    println!("[mu_list] After calling closure: {mu_list:?}");

    let list = vec![7, 8, 9];
    println!("Before defining closure: {list:?}");
    // Spawning a thread requires move.
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

fn closures_traits() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // Using an Fn closure with sort_by_key
    list.sort_by_key(|r| r.width);
    println!("sorted list:\n{list:#?}");

    // Using an FnMut closure with sort_by_key
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        println!("key call count: {}, current element: {}", num_sort_operations, r.width);
        r.width
    });

    println!("sorted list:\n{list:#?}");

    // Compile Error: attempting to use an FnOnce closure with sort_by_key
    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{list:#?}");
}

pub fn run() {
    // closures_demo();
    // closures_types();
    // closures_capturing_or_moving();
    closures_traits();
}