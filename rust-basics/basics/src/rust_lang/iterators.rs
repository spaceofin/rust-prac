#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn shoes_in_size_by_ref(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(|s| s.size == shoe_size).collect()
}


fn iterators_basic() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    println!("-----v1_iter-----");
    for val in v1_iter {
        println!("Got: {val}");
    }
    println!("v1: {v1:?}");
 
    let mut v2 = vec![4, 5, 6];
    let v2_iter_mut = v2.iter_mut();
    println!("-----v2_iter_mut-----");
    for val in v2_iter_mut {
        *val *= 10;
        println!("Got: {val}");
    }
    println!("v2: {v2:?}");

    let v3 = vec![7, 8, 9];
    let v3_into_iter = v3.into_iter();
    println!("-----v3_into_iter-----");
    for val in v3_into_iter {
        println!("Got: {val}");
    }
    // Compile Error: `v3` was moved into `v3_into_iter`.
    // println!("v3: {v3:?}");
    let v3 = vec![7, 8, 9];
    let v3_into_iter = v3.into_iter();
    for mut val in v3_into_iter {
        val *= 10;
        println!("Got: {val}");
    }

    let v4 = vec![10, 20, 30];
    let v4_iter = v4.iter();
    let total: i32 = v4_iter.sum();
    println!("v4 total: {total:?}");
    println!("V4: {v4:?}");
}

fn iterators_demo() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_clone: Vec<_> = v1.iter().map(|x| x).collect();
    let v1_plus: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    let v1_string: Vec<String> = v1.iter().map(|x| x.to_string()).collect();

    println!("v1_clone: {v1_clone:?}");
    println!("v1_plus: {v1_plus:?}");
    println!("v1_string: {v1_string:?}");

    let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

    let shoes_filter_13: Vec<_> = shoes.iter().filter(|s| s.size == 13).collect();
    println!("shoes_filter_13:\n{shoes_filter_13:#?}");
    let shoes_in_10_size_by_ref: Vec<_> = shoes_in_size_by_ref(&shoes, 10);
    println!("shoes_in_10_size_by_ref:\n{shoes_in_10_size_by_ref:#?}");
    println!("shoes:{shoes:?}");

    let shoes_in_10_size = shoes_in_size(shoes, 10);
    println!("shoes_in_10_size:\n{shoes_in_10_size:#?}");
    // Compile Error: value borrowed after move.
    // println!("shoes:{shoes:?}");
}

pub fn run() {
    // iterators_basic();
    iterators_demo();
}