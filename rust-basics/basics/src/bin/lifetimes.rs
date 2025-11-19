#![allow(dead_code)]
#![allow(unused_variables)]

// fn dangling_reference_example() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     } // x's lifetime has ended.
//     // Compile Error: r would be a dangling reference because x is dropped.
//     println!("r: {r}");
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
    title: &'a str,
    pages: Vec<usize>
}

fn generic_lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("[outer scope]The longest string is {result}");

    {
        let string2 = String::from("tuvwxyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("[inner scope]The longest string is {result}");
    }
}

// fn struct_lifetimes() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     let i = ImportantExcerpt {
//         part: first_sentence,
//         pages: vec![4,5]
//     };
//     println!("i: {i:#?}");
// }

fn struct_lifetimes() {
    let i;
    let title: &str = "Moby Dick";
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        i = ImportantExcerpt {
            part: first_sentence,
            title,
            pages: vec![4,5]
        };
        println!("[inner scope]i: {i:#?}");
    }
    // Compile Error: `i.part` references a value that has been dropped
    // println!("[outer scope]i: {i:#?}");
}

fn main() {
    generic_lifetimes();
    struct_lifetimes();
}