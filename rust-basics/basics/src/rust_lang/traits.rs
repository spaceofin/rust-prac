use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize_author(&self) -> String; 

    // fn summarize(&self) -> String;   

    // `summarize` method with default implementation.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

#[derive(Debug)]
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Default for NewsArticle {
    fn default() -> Self {
        Self {
            headline: String::from(""),
            location: String::from(""),
            author: String::from(""),
            content: String::from(""),
        }
    }
}

// impl Default for SocialPost {
//     fn default() -> Self {
//         Self {
//             username: String::from(""),
//             content: String::from(""),
//             reply: false,
//             repost: false,
//         }
//     }
// }

impl SocialPost {
    fn default() -> Self {
        Self {
            username: String::from(""),
            content: String::from(""),
            reply: false,
            repost: false,
        }
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Describe {
    fn describe(&self) -> String;
}

pub fn print_description(item: &impl Describe) {
    println!("{}", item.describe());
}

fn print_and_display_description<T: Describe + Debug>(item: &T) {
    println!("item: {:?}", item);
    println!("{}", item.describe());
}

// `impl Trait` syntax is syntax sugar for a trait bound.
// pub fn print_description<T: Describe>(item: &T) {
//     println!("{}", item.describe());
// }

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
}

struct Movie {
    title: String,
    director: String,
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("Book '{}' by {}", self.title, self.author)
    }
}

fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> () { 
    println!("some_function1 called: t is {t}, u is {u:?}");
}

// with `where1` clauses
fn some_function2<T, U>(t: &T, u: &U) -> ()
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("some_function1 called: t is {t}, u is {u:?}");
}

// Returns a type that implements the Describe trait
fn returns_descrizable() -> impl Describe {
    Book {
        title: String::from("New Picture"),
        author: String::from("Charlie")
    }
}

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

trait Print {
    fn print(&self) -> ();
}

// blanket implementation
impl<T: Debug> Print for T {
    fn print(&self) -> () {
        println!("print: {self:?}");
    }
}

fn trait_basic_examples() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };
    println!("article: {article:#?}");
    println!("post: {post:#?}");

    println!("1 new article: {}", article.summarize());
    println!("1 new post: {}", post.summarize());


    let default_article = NewsArticle::default();
    let default_post = SocialPost::default();

    println!("default_article: {default_article:#?}");
    println!("default_post: {default_post:#?}");

    let alice_article = NewsArticle {
        author: "Alice".into(),
        ..Default::default()
    };

    let mut alice_post = SocialPost::default();
    alice_post.username = "Alice".into();

    println!("alice_article: {alice_article:#?}");
    println!("post: {alice_post:#?}");
}

fn trait_examples() {
    let book = Book {title: "New World".into(), author:"Alice".into()};
    print_description(&book);
    print_and_display_description(&book);

    let movie = Movie {title: "New Music".into(), director:"Bob".into()};
    // Compile Error: the trait `Describe` is not implemented for `Movie`
    // print_description(&movie);

    let number1= 10;
    let number2 = 20;
    some_function1(&number1, &number2);
    some_function2(&number1, &number2);

    let pair1 = Pair {x: 2, y: 3};
    println!("pair1: {pair1:?}");
    let pair2 = Pair::new(10, 20);
    println!("pair2: {pair2:?}");

    pair1.cmp_display();
    pair2.cmp_display();

    let a = 32;
    a.print();
    let b = "Hellooooo".to_string();
    b.print();
}

pub fn run() {
    // trait_basic_examples();
    trait_examples();
}