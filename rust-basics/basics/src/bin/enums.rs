#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum MyIpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct MyIpAddr {
    kind: MyIpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrType1 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrType2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit the program");
            }
            Message::Move { x, y } => {
                println!("Move to coordinates: ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("Write message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to RGB({}, {}, {})", r, g, b);
            }
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() { println!("add fancy hat!"); }
fn remove_fancy_hat() { println!("remove fancy hat!"); }
fn move_player(num_spaces: u8) { println!("move player {num_spaces}"); }
fn reroll() { println!("reroll!"); }

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // let state = if let Coin::Quarter(state) = coin {
    //     state
    // } else {
    //     return None;
    // };

    let Coin::Quarter(state) = coin else {
        return None;
    };
    println!("state: {state:?}");

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn enum_basics() {
    let four = MyIpAddrKind::V4;
    let six = MyIpAddrKind::V6;
    println!("four: {four:?}, six: {six:?}");

    let home = MyIpAddr {
        kind: MyIpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = MyIpAddr {
        kind: MyIpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home: {home:?}, address: {loopback:?}");

    let home = IpAddrType1::V4(String::from("127.0.0.1"));
    let loopback = IpAddrType1::V6(String::from("::1"));
    println!("home: {home:?}, address: {loopback:?}");

    let home = IpAddrType2::V4(127, 0, 0, 1);
    let loopback = IpAddrType2::V6(String::from("::1"));
    println!("home: {home:?}, address: {loopback:?}");

    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let loopback = IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1));
    println!("home: {home:?}, address: {loopback:?}");

    
    let q = Message::Quit;
    let v = Message::Move {x:2, y:3};
    let m = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(255, 165, 0);
    q.call();
    v.call();
    m.call();
    c.call();
}

fn match_examples() {
    let coin1 = Coin::Dime;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Penny;
    let coin4 = Coin::Quarter(UsState::Alaska);

    println!("value in {coin1:?}: {:?}\n", value_in_cents(&coin1));
    println!("value in {coin2:?}: {:?}\n", value_in_cents(&coin2));
    println!("value in {coin3:?}: {:?}\n", value_in_cents(&coin3));
    println!("value in {coin4:?}: {:?}\n", value_in_cents(&coin4));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {five:?}, six: {six:?}, none: {none:?}");

    println!("five: {}, six: {}, none: {}", 
        five.map_or("None".to_string(), |v| v.to_string()),
        six.map_or("None".to_string(), |v| v.to_string()),
      
        none.map_or("None".to_string(), |v| v.to_string()),
    );
    println!();

    println!("__dice_roll1__");
    let dice_roll1 = 9;
    match dice_roll1 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other matches any remaining value and binds it to the variable `other`
        other => move_player(other),
    }

    println!("__dice_roll2__");
    let dice_roll2 = 9;
    match dice_roll2 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // `_` matches any remaining value but does not bind it to a variable
        _ => reroll(),
        // empty tuple `()` does nothing
        // _ => (),       
    }
}

fn let_examples () {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    // let coin = Coin::Dime;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
    println!("count: {count}");

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    // let coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
    println!("count: {count}");

    let coin = Coin::Quarter(UsState::Alabama);
    // let coin = Coin::Dime;
    describe_state_quarter(coin);
}

fn main() {
    // enum_basics();
    // match_examples();
    let_examples();
}