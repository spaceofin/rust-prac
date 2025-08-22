use axum::{Router, routing::{get,post}, extract::{Path, Query, Json, Form}};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
  id: Option<i32>,
  name: Option<String>
}


async fn greet_with_id(Path(id): Path<i32>) -> String {
  format!("Welcome, number {}", id)
}

async fn greet_with_name(Path(name): Path<String>) -> String {
  format!("Welcome, {}", name)
}

async fn greet(Path((id,name)): Path<(i32, String)>) -> String {
  format!("Welcome, number {} {}", id, name)
}

async fn greet_with_id_query(Query(user): Query<User>) -> String {
  format!("Welcome, number {}", user.id.unwrap_or(-1))
}

async fn greet_with_name_query(Query(user): Query<User>) -> String {
  format!("Welcome, {}",  user.name.clone().unwrap_or("Anonymous".to_string()))
}

async fn greet_with_query(Query(user): Query<User>) -> String {
  format!("Welcome, number {} {}", 
    user.id.unwrap_or(-1), 
    user.name.as_ref().unwrap_or(&"Anonymous".to_string())
  )
}

// Content-Type: text/plain
async fn user_name_info(name: String) -> String {
  format!("user name is {}", name)
}

// Content-Type: text/plain
async fn user_id_info(id: String) -> String {
      match id.parse::<u32>() {
        Ok(id) => format!("user id is {}", id),
        Err(_) => "Invalid id".to_string(),
    }
}

// Content-Type: application/json
async fn user_info_json(Json(user): Json<User>) -> String {
  format!("id: {}, name: {}",user.id.unwrap_or(-1), user.name.clone().unwrap_or("Anonymous".to_string()))
}

// Content-Type: application/x-www-form-urlencoded
async fn user_info_form(Form(user): Form<User>) -> String {
  format!("id: {}, name: {}",user.id.unwrap_or(-1), user.name.clone().unwrap_or("Anonymous".to_string()))
}



pub fn create_router() -> Router {
  Router::new()
    .nest("/greet",
      Router::new()
        .route("/id/{id}", get(greet_with_id))
        .route("/name/{name}", get(greet_with_name))
        .route("/{id}/{name}", get(greet))
        .route("/id", get(greet_with_id_query))
        .route("/name", get(greet_with_name_query))
        .route("/", get(greet_with_query))
    )
    .nest("/info",
      Router::new()
          .route("/id", post(user_id_info))
          .route("/name", post(user_name_info))
          .route("/json", post(user_info_json))
          .route("/form", post(user_info_form))
    )
}