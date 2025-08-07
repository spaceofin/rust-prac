use serde::{Serialize, Deserialize};
use serde_json;
use std::any::type_name;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn sample_user() -> User {
    User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    }
}

// JSON formatted string
const SAMPLE_USER_JSON: &str = r#"
{
    "id": 2,
    "name": "Sophia",
    "email": "sophia@example.com"
}
"#;

fn print_type_of<T>(var_name: &str, _: &T) {
    println!("[Type of {}] {}", var_name, type_name::<T>());
}

fn main() {
    let user1 = sample_user();
  
    let serialized_user1_json = serde_json::to_string_pretty(&user1).unwrap();
    println!("Serialized JSON:\n{}\n", serialized_user1_json);

    let deserialized_user1: User = serde_json::from_str(&serialized_user1_json).unwrap();
    println!("Deserialized struct:\n{:?}\n", deserialized_user1);

    let deserialized_user2: User = serde_json::from_str(SAMPLE_USER_JSON).unwrap();
    println!("Deserialized struct:\n{:?}\n", deserialized_user2);

    println!("__________types__________");
    print_type_of("user1", &user1);
    print_type_of("serialized_user1_json", &serialized_user1_json);
    print_type_of("deserialized_user1", &deserialized_user1);
    print_type_of("deserialized_user2", &deserialized_user2);

}