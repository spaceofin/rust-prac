#![allow(unused_variables)]
#![allow(dead_code)]

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

fn main() {
    enum_basics();
}