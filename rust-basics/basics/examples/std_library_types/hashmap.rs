use std::collections::{HashMap,HashSet};
use std::any::type_name;
use std::io::{self, Write};

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

// Eq requires that you derive PartialEq on the type.
#[derive(PartialEq, Eq, Hash)]
struct Account<'a>{
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a>{
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>,
        username: &'a str, password: &'a str){
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon = Account {
        username,
        password,
    };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        },
        _ => println!("Login failed!"),
    }
}


fn hashmap_varied_keys() {
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "psasword123");
    try_logon(&accounts, "j.everyman", "password123");

    //

    println!();
    println!("--------------------");

    let test_account = Account { 
        username: "test", 
        password: "test1234" 
    };
    let test_account_info = AccountInfo { 
        name: "test", 
        email: "test@email.com"
    };

    let mut test_accounts: Accounts = HashMap::new();
    test_accounts.insert(test_account, test_account_info);

    let mut username_input = String::new();
    let mut password_input = String::new();

    print!("Enter username: ");
    io::stdout().flush().unwrap(); 
    io::stdin()
        .read_line(&mut username_input)
        .expect("Failed to read username");

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut password_input)
        .expect("Failed to read password");

    let username = username_input.trim();
    let password = password_input.trim();

    try_logon(&test_accounts, username, password);
}


fn try_insert(set: &mut HashSet<i32>, set_name: &str, value: i32) {
    set.insert(value)
        .then(|| println!("[{}] Insert {} succeeded!", set_name, value))
        .unwrap_or_else(|| println!("[{}] Insert {} failed! Value already exists. Current set: {:?}", set_name, value, set));
}

fn try_contains(set: &HashSet<i32>, set_name: &str, value: i32) {
    set.contains(&value)
        .then(|| println!("[{}] Value {} is in the set. Current set: {:?}", set_name, value, set))
        .unwrap_or_else(|| println!("[{}] Value {} is not in the set. Current set: {:?}", set_name, value, set));
}

fn hashset_examples() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    try_insert(&mut a, "a", 4);
    try_contains(&a, "a", 4);

    try_insert(&mut b, "b", 4);
    try_insert(&mut b, "b", 5);
    try_contains(&b, "b", 1);
    try_contains(&b, "b", 5);

    // If a collection's element type implements `Debug`,
    // then the collection implements `Debug`.
    // It usually prints its elements in the format `[elem1, elem2, ...]`
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Print results in arbitrary order
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());
    println!("Symmetric Difference: {:?}",
             a.symmetric_difference(&b).collect::<Vec<&i32>>());
}

pub fn hashmap_demo() {
    // hashmap_basic();
    // hashmap_varied_keys();
    hashset_examples();
}