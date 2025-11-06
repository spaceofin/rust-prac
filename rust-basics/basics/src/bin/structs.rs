#![allow(dead_code)]

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

// Compile Error: missing lifetime specifier
// struct Book {
//     title: &str,
//     author: &str,
//     pages: u32,
// }

#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
    author: &'a str,
    pages: u32,
}

fn structs_basic() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("user1: {:?}", user1);
    user1.email = String::from("anotheremail@example.com");
    println!("user1: {:?}", user1);

    let user2 = build_user(
        "user2@example.com".to_string(),
        "user2".to_string());
    println!("user2: {:?}", user2);

    let user3 = User {
        active: user1.active,
        username: user1.username.clone(),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count.clone(),
    };
    println!("user3: {:?}", user3);

    let user4 = User {
        username: "user4".to_string(),
        ..user1
    };
    println!("user4: {:?}", user4);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:?}, origin: {:?}", black, origin);

    let subject = AlwaysEqual;
    println!("subject: {:?}", subject);

    let book = Book {
        title: "The Rust",
        author: "Rustean",
        pages: 123,
    };
    println!("book: {:?}",book);
}

fn main() {
    structs_basic();
}