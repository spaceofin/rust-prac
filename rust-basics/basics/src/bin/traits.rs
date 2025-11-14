
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

fn main() {
    trait_basic_examples();
}