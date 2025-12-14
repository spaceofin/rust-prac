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
    println!("User {} has green hair", user.1);
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



fn select_users_with_hasquery(connection: &mut SqliteConnection) {

  #[derive(HasQuery)]
  #[diesel(table_name = self::schema::users)]
  struct User {
      name: String,
      hair_color: Option<String>,
      id: i32,
  }

// This now returns a `Vec<User>`, which is automatically infered
// by using `User::query()` to construct your query
  let users_with_green_hair = User::query()
      .filter(hair_color.eq("green"))
      .order_by(updated_at.desc())
      .load(connection)
      .expect("failed to load users");

  for user in users_with_green_hair {
    println!("User {} has green hair", user.name);
  }
   println!();


  use diesel::sql_types::{Nullable, Integer};
  use diesel::dsl::sql;
  use diesel::expression::SqlLiteral;

  #[derive(HasQuery)]
  #[diesel(table_name = self::schema::users)]
  #[diesel(base_query = users.order_by(updated_at.asc()))]
  struct CustomUser {
      id: i32,
      name: String,
      hair_color: Option<String>,
      // This does not work: it is an AggregateExpression, not a SelectableExpression.
      // #[diesel(select_expression = lag(id).partition_by(hair_color))]
      #[diesel(select_expression = sql::<Nullable<Integer>>("LAG(id) OVER (PARTITION BY hair_color)"))]
      #[diesel(select_expression_type = SqlLiteral<Nullable<Integer>>)]
      last_user_with_same_hair_color: Option<i32>,
  }
  
  let custom_users = CustomUser::query().load(connection).expect("failed to load users");

  for user in custom_users {
      println!("User {} has the hair color {:?}", user.name, user.hair_color);
      if let Some(lag_id) = user.last_user_with_same_hair_color {
          println!("->The user with {lag_id} the has the same hair color");
      }
  }
}

fn result_mapping(connection: &mut SqliteConnection) {
  let ids = users.select(id)
    .load::<i32>(connection)
    .expect("failed to load users");
  println!("\nids:\n{ids:?}");

  // Single element tuple results
  let names = users.select((name,))
    .load::<(String,)>(connection)
    .expect("failed to load users");
  println!("names:\n{names:?}");
  let ids_and_names = users.select((id, name))
    .load::<(i32, String)>(connection)
    .expect("failed to load users");
  println!("\nids and names:\n{ids_and_names:?}");

  let nested_result = users.select((id, name, (id, name)))
    .load::<(i32, String, (i32, String))>(connection)
    .expect("failed to load users");
  println!("\nnested_result:\n{nested_result:?}");

  #[derive(Queryable, Debug)]
  struct UserCore {
    id: i32,
    name: String,
  }

  let user_core_infos = users.select((id, name))
    .load::<UserCore>(connection)
    .expect("failed to load users");
  println!("\nuser_core_infos:\n{user_core_infos:?}");

  #[derive(Queryable, Selectable, Debug)]
  #[diesel(table_name = self::schema::users)]
  #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
  struct UserCoreSelectable {
    id: i32,
    name: String,
  }

  let user_infos = users.select(UserCoreSelectable::as_select())
    .load(connection)
    .expect("failed to load users");
  println!("\nuser_infos:\n{user_infos:?}");

  #[derive(HasQuery, Debug)]
  #[diesel(table_name = self::schema::users)]
  struct UserForHasQuery {
      id: i32,
      name: String,
  }

  let user_hasquery_infos = UserForHasQuery::query()
    .load(connection)
    .expect("failed to load users");
  println!("\nuser_hasquery_infos:\n{user_hasquery_infos:?}");

}

  

fn main() {
  let connection = &mut establish_connection();
  // select_version(connection);
  // select_all_users(connection);
  // select_users_with_querydsl(connection);
  // select_users_with_hasquery(connection);
  result_mapping(connection);
}