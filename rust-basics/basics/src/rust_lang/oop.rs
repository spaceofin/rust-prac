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

pub fn run() {
    encapsulation_example();
}