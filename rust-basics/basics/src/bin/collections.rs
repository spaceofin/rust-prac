#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_examples() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let v2 = vec![4,5,6];
    println!("v1: {v1:?}, v2: {v2:?}");

    let third: &i32 = &v1[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v3 = vec![1, 2, 3, 4, 5];
    // Compile Error: Referencing a nonexistent element causes a panic.
    // let does_not_exist = &v3[100]; 
    // `get` method returns `None` without panicking.
    let does_not_exist = v3.get(100);
    println!("does_not_exist: {does_not_exist:?}");

    let v4 = vec![100, 32, 57];
    for i in &v4 {
        print!("{i} ");
    }
    println!();

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }
    println!("v4: {v4:?}");

    let third = &mut v4[2];
    *third += 50;
    println!("v4: {v4:?}");
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row: {row:?}");

    {
        let v = vec![1, 2, 3, 4];
    }
    // Compile Error: cannot find value `v` in this scope.
    // println!("v: {v:?}");
}

fn string_examples() {
    let s1 = String::new();
    let data = "initial contents";
    let s2 = data.to_string();
    // The method also works on a literal directly
    let s3 = "initial contents".to_string();
    println!("s1: {s1:?}");
    println!("s2: {s2:?}");
    println!("s3: {s3:?}");

    // Strings are UTF-8 encoded.
    let greetings: [String; 11] = [
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שלום"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];
    greetings.iter().for_each(|hello| println!("{hello:?}"));

    let mut s4 = String::from("foo");
    s4.push_str("bar");
    let mut s5 = String::from("foo");
    let s6 = "bar";
    // `push_str` take string slice(`&str`) and does not take ownership.
    s5.push_str(s6);
    println!("s4: {s4:?}");
    println!("s5: {s5:?}");
    println!("s6: {s6:?}");

    let mut s7 = String::from("lo");
    // `push` method takse a single character.
    s7.push('l');
    println!("s7: {s7:?}");

    let s8 = String::from("Hello, ");
    let s9 = String::from("world!");
    let s10 = s8 + &s9;
    // println!("s8: {s8:?}"); // Compile Error: value borrowed after move.
    println!("s9: {s9:?}");
    println!("s10: {s10:?}");

    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");
    let s14 = s11 + "-" + &s12 + "-" + &s13;
    let s11 = String::from("tic");
    let s15 = format!("{s11}-{s12}-{s13}");
    println!("s14: {s14:?}");
    println!("s15: {s15:?}");

    // Compile Error: String don't support indexing.
    // let h = s14[0];

    let hello1 = String::from("hello");
    let hello2 = String::from("안녕하세요");
    println!("hello1: {}{}", &hello1[0..1], &hello1[1..2]);
    println!("hello2: {}{}", &hello2[0..3], &hello2[3..6]); 
    // Compile Error
    // println!("hello1: {}", &hello1[0]);
    // println!("hello2: {}", &hello2[0..2]);

    for c in "가나다".chars() { print!("{c} "); }
    println!();
    for b in "가나다".bytes() { print!("{b} "); }
}

fn hashmap_examples() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {scores:?}");

    let team_blue = String::from("Blue");
    // let score = scores.get(&team_name).unwrap_or(&0);
    let score = scores.get(&team_blue).copied().unwrap_or(0);
    println!("score for team Blue: {score:?}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Compile Error: value borrowed after move.
    // println!("field_name: {field_name}, field_value: {field_value}")

    scores.insert(String::from("Blue"), 25);
    println!("score for team Blue: {:?}", &scores.get(&team_blue).copied().unwrap_or(0));

    scores.entry(String::from("Orange")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(30);
    println!("scores: {scores:?}");

    
    let text = "hello world wonderful world";
    let mut word_map = HashMap::new();
    for word in text.split_whitespace() {
        // Get a mutable reference to the value for this word.
        let count_value = word_map.entry(word).or_insert(0);
        *count_value += 1;
    }

    println!("{word_map:?}");
}

fn main() {
    // vector_examples();
    // string_examples();
    hashmap_examples();
}