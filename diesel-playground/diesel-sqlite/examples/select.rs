#![allow(unused_imports)]
#![allow(dead_code)]

use diesel::prelude::*;
use diesel_sqlite::*;
use chrono::NaiveDateTime;
use self::schema::users::dsl::*;
use diesel::dsl::lag;

// Declare the `version()` SQL function
#[declare_sql_function]
extern "SQL" {
    // fn version() -> Text;
    fn sqlite_version() -> Text;
}

fn select_version(connection: &mut SqliteConnection) {
  let version_text = diesel::select(sqlite_version()).get_result::<String>(connection).expect("failed to get version");
  println!("Running SQLite: `{version_text}`");
}

fn select_all_users(connection: &mut SqliteConnection) {
  // This returns a Vec<(i32, String, Option<String>, NaiveDateTime, NaiveDateTime)>
  let all_users = users.load::<(i32, String, Option<String>, NaiveDateTime, NaiveDateTime)>(connection).expect("failed to load users");

  for user in all_users {
      println!("{}: {}", user.0, user.1); 
  }
}

fn select_users_with_querydsl(connection: &mut SqliteConnection) {
  let users_with_green_hair = users.filter(hair_color.eq("green"))
  .order_by(updated_at.desc())
  .load::<(i32, String, Option<String>, NaiveDateTime, NaiveDateTime)>(connection).expect("failed to load users");

  for user in users_with_green_hair {
    println!("User {} has grean hair", user.1);
  }
   println!();

  let users_with_no_hair_color = users.filter(hair_color.is_null())
  .select(name)
  .load::<String>(connection)
  .expect("failed to load users");

  for user_name in users_with_no_hair_color {
    println!("User {} has no hair color", user_name);
  }
  println!();

  let user_rows = users.select((
    id, 
    name, 
    hair_color, 
    lag(id).partition_by(hair_color)
  )).order_by(updated_at.asc())
  .load::<(i32, String, Option<String>, Option<i32>)>(connection)
  .expect("failed to load users");

  for user in user_rows {
    println!("User {} has the hair color {:?}", user.1, user.2);
    if let Some(lag_id) = user.3 {
        println!("->The user with the {lag_id} has the same hair color");
    }
  }
}

fn main() {
  let connection = &mut establish_connection();
  // select_version(connection);
  // select_all_users(connection);
  select_users_with_querydsl(connection);
}