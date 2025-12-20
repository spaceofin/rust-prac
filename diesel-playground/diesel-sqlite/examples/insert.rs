#![allow(dead_code)]
#![allow(unused_variables)]

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_sqlite::*;
use diesel::{debug_query, sqlite::Sqlite};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Queryable, PartialEq)]
#[diesel(table_name = users)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = self::schema::users)]
pub struct UserForm<'a> {
    name: &'a str,
    hair_color: Option<&'a str>,
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
    use self::schema::users::dsl::*;

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

fn batch_insert(connection: &mut SqliteConnection) {
    use self::schema::users::dsl::*;

    // let values = &vec![name.eq("Sean"), name.eq("Tess")];
    // let values = &vec![Some(name.eq("Sean")), None];
    // let values = &vec![
    //     (name.eq("Sean"), hair_color.eq("Black")),
    //     (name.eq("Tess"), hair_color.eq("Brown")),
    // ];
    // let values = &vec![
    //     (name.eq("Sean"), Some(hair_color.eq("Black"))),
    //     (name.eq("Ruby"), None),
    // ];

    let json = r#"[
        { "name": "Sean", "hair_color": "Black" },
        { "name": "Tess", "hair_color": "Brown" }
    ]"#;
    let user_form = serde_json::from_str::<Vec<UserForm>>(json)
    .expect("failed to parse UserForm");

    let query = diesel::insert_into(users)
        .values(&user_form);
    println!("query:\n{}",debug_query::<Sqlite, _>(&query));
    let inserted_users = query
        .get_results::<User>(connection)
        .expect("failed to insert users");
    println!("inserted_users:\n{inserted_users:?}");
}   

fn returning_clause(connection: &mut SqliteConnection) {
    use diesel::select;
    use self::schema::users::dsl::*;

    let now = select(diesel::dsl::now)
        .get_result::<NaiveDateTime>(connection)
        .expect("failed to fetch current timestamp from database");

    let inserted_users: Vec<User> = diesel::insert_into(users)
        .values(&vec![
            (id.eq(100), name.eq("Sean")),
            (id.eq(200), name.eq("Tess")),
        ])
        .get_results(connection)
        .expect("failed to insert users");

    let expected_users = vec![
        User {
            id: 100,
            name: "Sean".into(),
            hair_color: None,
            created_at: now,
            updated_at: now,
        },
        User {
            id: 200,
            name: "Tess".into(),
            hair_color: None,
            created_at: now,
            updated_at: now,
        },
    ];
    assert_eq!(expected_users, inserted_users);

    println!("inserted_users:\n{inserted_users:?}");
    println!("expected_users:\n{expected_users:?}");

    let inserted_user = diesel::insert_into(users)
    .values((id.eq(300), name.eq("Ruby")))
    .get_result(connection)
    .expect("failed to insert users");

    let expected_user = User {
        id: 300,
        name: "Ruby".into(),
        hair_color: None,
        created_at: now,
        updated_at: now,
    };
    assert_eq!(expected_user, inserted_user);

    println!("inserted_user:\n{inserted_user:?}");
    println!("expected_user:\n{expected_user:?}");

    let inserted_user_id = diesel::insert_into(users)
        .values(name.eq("Ruby"))
        .returning(id)
        .get_result::<i32>(connection)
        .expect("failed to insert user");
    
    println!("inserted_user_id: {inserted_user_id:?}");
}

fn main() {
  let connection = &mut establish_connection();
    // insert_defaults(connection);
    // insert_user_with_name(connection, "Sean");
    // insert_user_with_name_and_haircolor(connection, "Tess", "Brown");
    // insert_with_insertable(connection);
    // batch_insert(connection);
    returning_clause(connection);
}