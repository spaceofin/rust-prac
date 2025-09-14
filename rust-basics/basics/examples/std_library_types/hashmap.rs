use std::collections::HashMap;
use std::any::type_name;

fn hashmap_basic() {
    let mut students = HashMap::new();
    println!("[initial hashmap] len: {}",students.len());

    students.insert(1,"Daniel");
    students.insert(22,"Ashley");
    students.insert(333,"Katie");
    let insert_result = students.insert(4444,"Robert");
    println!("insert_result: {:?}",insert_result);

    println!("[hashmap after insert] len: {}",students.len());

    // Takes a reference and returns Option<&V>
    match students.get(&333) {
        Some(name) => println!("student with ID 333 is {}", name),
        _ => println!("No student found with ID 333"),
    }

    let insert_result = students.insert(333, "Alice");
    println!("insert_result: {:?}",insert_result);
    match students.get(&333) {
        Some(name) => println!("student with ID 333 is {}", name),
        _ => println!("No student found with ID 333"),
    }

    println!();
    for (id, name) in students.iter() {
        print!("[{}]{} ", id, name); 
    }
    println!(); 
    println!();

    students.remove(&22);
    println!("remove student ID 22");
    println!();
    for (id, name) in students.iter() {
        print!("[{}]{} ", id, name); 
    }
    println!();
}

pub fn hashmap_demo() {
    hashmap_basic();
}