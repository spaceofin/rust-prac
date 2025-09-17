use std::collections::HashMap;
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

pub fn hashmap_demo() {
    // hashmap_basic();
    hashmap_varied_keys();
}