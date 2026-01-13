mod collection_module {
    #[derive(Debug)]
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn new() -> Self {
            Self {
                list: vec![],
                average: 0.0,
            }
        }

        pub fn from_list(list: Vec<i32>) -> Self {
            let mut col = Self {
                list,
                average: 0.0,
            };
            col.update_average();
            col
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

fn encapsulation_example() {
    use collection_module::AveragedCollection;

    let li = vec![1,2,3,4,5];
    
    // Compile Error: 'list' and 'average' fields are private.
    // let mut collection = AveragedCollection { list: li, average: 3.0 };

    let mut collection = AveragedCollection::from_list(li);
    println!("collection: {collection:?}");
}


mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // Trait objects are created by specifying a pointer, such as a reference 
        // or a Box<T> smart pointer, followed by the 'dyn' keyword and the trait name.
        pub components: Vec<Box<dyn Draw>>,
    }
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!(
                "Drawing Button (width: {}, height: {}, label: {:?})",
                self.width, self.height, self.label
            );
            // code to actually draw a button
        }
    }

    pub struct StaticScreen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> StaticScreen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}


fn trait_objects_example() {
    use gui::{Button, Screen, Draw};

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!(
                "Drawing SelectBox (width: {}, height: {}, options: {:?})",
                self.width, self.height, self.options
            );
            // code to actually draw a select box
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();

    // Compile Error: the trait bound `String: Draw` is not satisfied.
    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };
    // screen.run();
}

fn generics_example() {
    use gui::{Button, StaticScreen, Draw};

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!(
            "Drawing SelectBox (width: {}, height: {}, options: {:?})",
            self.width, self.height, self.options
        );
            // code to actually draw a select box
        }
    }

    let screen1 = StaticScreen {
        components: vec![
            SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            },
            SelectBox {
                width: 100,
                height: 20,
                options: vec![
                    String::from("No"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            },
        ],
    };
    screen1.run();

    let screen2 = StaticScreen {
        components: vec![
            Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            },
            Button {
                width: 70,
                height: 15,
                label: String::from("CANCEL"),
            },
        ],
    };
    screen2.run();
}

// mod blog {
//     enum State {
//         Draft,
//         Review,
//         Published
//     }
//     pub struct Post {
//         state: State,
//         draft_content: String,
//         content: String
//     }

//     impl Post {
//         pub fn new() -> Self {
//             Self {
//                 state: State::Draft,
//                 draft_content: String::new(),
//                 content: String::new(),
//             }
//         }

//         pub fn add_text(&mut self, text: &str) {
//             self.draft_content = text.to_string();
//         }

//         pub fn request_review(&mut self) {
//             match self.state {
//                 State::Draft if !self.draft_content.is_empty() => {
//                     self.state = State::Review;
//                 },
//                 _ => {}
//             }
//         }

//         pub fn approve(&mut self) {
//             if let State::Review = self.state {
//                 self.content = self.draft_content.clone();
//                 self.draft_content.clear();
//                 self.state = State::Published;
//             }
//         }

//         pub fn content(&self) -> &str {
//             &self.content
//         }
//     }
// }

mod blog {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
        
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

}


fn oop_pattern_example() {
    use blog::Post;

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub fn run() {
    // encapsulation_example();
    // trait_objects_example();
    // generics_example();
    oop_pattern_example();
}   