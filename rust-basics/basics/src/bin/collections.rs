#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

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

fn main() {
    vector_examples();
}