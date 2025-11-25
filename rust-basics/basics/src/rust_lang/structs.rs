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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct Square {
    side: u32,
}

// Each struct is allowed to have multiple `impl` blocks.
impl Square {
    // method
    fn area(&self) -> u32 {
        self.side * self.side
    }
    // associated function
    fn info() -> () {
        println!("This is a Square struct.");
    }
}

impl Square {
    fn perimeter(&self) -> u32 {
        4 * self.side
    }
}

fn structs_examples() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1: {rect1:#?}");
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(40 * scale),
        height: 60,
    };

    // dbg!(&rect2);
    dbg!(rect2);
    // Compile Error: `dbg!` takes ownership of `rect2`,
    // println!("rect2: {rect2:#?}");

    let sq1 = Square { side: 30 };
    println!("sq: {sq1:#?}");
    println!(
        "The area of the square is {} square pixels, and its perimeter is {} pixels.",
        sq1.area(), sq1.perimeter()
    );

    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect4 = Rectangle {
        width: 20,
        height: 30,
    };
    let rect5 = Rectangle {
        width: 30,
        height: 30,
    };
    println!("Can rect5 hold rect3? {}", rect5.can_hold(&rect3));
    println!("Can rect5 hold rect4? {}", rect5.can_hold(&rect4));

    Square::info();

}

pub fn run() {
    println!("\n----------Structs Basic----------");
    structs_basic();
    println!("\n----------Structs Examples----------");
    structs_examples();
}