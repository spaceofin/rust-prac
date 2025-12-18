#![allow(dead_code)]
#![allow(unused_variables)]

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_sqlite::*;
use diesel::{debug_query, sqlite::Sqlite};

#[derive(Debug, Queryable)]
#[diesel(table_name = users)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

fn insert_defaults(connection: &mut SqliteConnection) {
    use self::schema::users::dsl::*;
    let insert_defaults_query = diesel::insert_into(users).default_values();
    println!("insert defaults:\n{}",debug_query::<Sqlite, _>(&insert_defaults_query));

    let inserted_default_user = insert_defaults_query
        .get_result::<User>(connection)
        .expect("failed to insert default user");
    println!("inserted_default_user:\n{inserted_default_user:?}")
}

fn insert_user_with_name(connection: &mut SqliteConnection, user_name: &str) {
    use self::schema::users::dsl::*;
    let insert_user = diesel::insert_into(users).values(name.eq(user_name));
    println!("insert user with name:\n{}",debug_query::<Sqlite, _>(&insert_user));

    let inserted_user = insert_user
        .get_result::<User>(connection)
        .expect("failed to insert user");
    println!("inserted_user:\n{inserted_user:?}")
}

fn insert_user_with_name_and_haircolor(connection: &mut SqliteConnection, user_name: &str, user_haircolor: &str) {
    use self::schema::users::dsl::*;
    let insert_user = diesel::insert_into(users).values((name.eq(user_name), hair_color.eq(user_haircolor)));
    println!("insert user with name and hair color:\n{}",debug_query::<Sqlite, _>(&insert_user));

    let inserted_user = insert_user
        .get_result::<User>(connection)
        .expect("failed to insert user");
    println!("inserted_user:\n{inserted_user:?}")
}

fn insert_with_insertable(connection: &mut SqliteConnection) {
    use serde::Deserialize;
    use serde_json;
    use self::schema::users::dsl::*;

    #[derive(Debug, Deserialize, Insertable)]
    #[diesel(table_name = self::schema::users)]
    pub struct UserForm<'a> {
        name: &'a str,
        hair_color: Option<&'a str>,
    }

    // let json = r#"{ "name": "Sean", "hair_color": "Black" }"#;
    // println!("json:\n{json}");
    // let user_form = serde_json::from_str::<UserForm>(json).expect("failed to parse UserForm");
    // println!("user_form:\n{user_form:?}");

    // let inserted_user = diesel::insert_into(users).values(&user_form)
    //     .get_result::<User>(connection)
    //     .expect("failed to insert user");
    // println!("inserted_user:\n{inserted_user:?}");

    //If one of the fields is None, the default value will be inserted for that field.
    let json = r#"{ "name": "Ruby", "hair_color": null }"#;
    println!("json:\n{json}");
    let user_form = serde_json::from_str::<UserForm>(json).expect("failed to parse UserForm");
    println!("user_form:\n{user_form:?}");

    let insert_user_query = diesel::insert_into(users).values(&user_form);
    println!("insert user query:\n{}",debug_query::<Sqlite, _>(&insert_user_query));
    let inserted_user = insert_user_query
        .get_result::<User>(connection)
        .expect("failed to insert user");
    println!("inserted_user:\n{inserted_user:?}");
}

fn main() {
  let connection = &mut establish_connection();
    // insert_defaults(connection);
    // insert_user_with_name(connection, "Sean");
    // insert_user_with_name_and_haircolor(connection, "Tess", "Brown");
    insert_with_insertable(connection);
}