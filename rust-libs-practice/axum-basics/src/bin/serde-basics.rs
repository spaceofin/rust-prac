#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeStruct};
use serde::de::{self, Visitor, MapAccess};
use serde_json;
use std::any::type_name;
use std::fmt;

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

#[derive(Debug)]
struct Person { name: String, age: u8, phones: Vec<String>, }

impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Person", 3)?;
        s.serialize_field("name", &self.name.to_uppercase())?;
        let age_str = format!("{:03}", &self.age);
        s.serialize_field("age", &age_str)?;
        let cleaned_phones: Vec<String> = (&self.phones)
            .iter()
            .map(|p| p.replace("-",""))
            .collect();
        s.serialize_field("phones", &cleaned_phones)?;
        s.end()
    }
}

struct PersonVisitor;

impl<'de> Visitor<'de> for PersonVisitor {
    type Value = Person;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("struct Person")
    }

   fn visit_map<V>(self, mut map: V) -> Result<Person, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut name = None;
        let mut age = None;
        let mut phones = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "name" => name = Some(map.next_value()?),
                "age" => {
                    let age_str: String = map.next_value()?;
                    let parsed_age = age_str
                        .parse::<u8>()
                        .map_err(de::Error::custom)?;
                    age = Some(parsed_age);
                }
                "phones" => {
                    let raw_phones: Vec<String> = map.next_value()?;
                    let formatted_phones: Vec<String> = raw_phones
                        .into_iter()
                        .map(|p| {
                            if p.len() == 11 {
                                format!("{}-{}-{}", &p[0..3], &p[3..7], &p[7..11])
                            } else if p.len() == 10 {
                                format!("{}-{}-{}", &p[0..2], &p[2..6], &p[6..10])
                            } else if p.len() == 9 {
                                format!("{}-{}-{}", &p[0..2], &p[2..5], &p[5..9])
                            } else {
                                p
                            }
                        })
                        .collect();
                    phones = Some(formatted_phones);
                }
                _ => { let _ : de::IgnoredAny = map.next_value()?; }
            }
        }

        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let age = age.ok_or_else(|| de::Error::missing_field("age"))?;
        let phones = phones.ok_or_else(|| de::Error::missing_field("phones"))?;

        Ok(Person { name, age, phones })
    }
}

impl<'de> Deserialize<'de> for Person {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("Person", &["name", "age", "phones"], PersonVisitor)
    }
}

fn serde_examples() {
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

fn person_serde_demo() {
    let person1 = Person { 
        name: "Alice".into(), 
        age: 22, 
        phones: vec!["000-0000-0000".into(), "11-1111-1111".into(), "22-222-2222".into()],
    };
    println!("__person1__\n{person1:?}");
    
    let serialized_person1 = serde_json::to_string(&person1).unwrap();
    println!("__person1 serialized__\n{:?}", serialized_person1);
    
    let deserialized_person1: Person = serde_json::from_str(&serialized_person1).unwrap();
    println!("__person1 deserialized__\n{:#?}", deserialized_person1);
}

fn main() {
    // serde_examples();
    person_serde_demo();
}