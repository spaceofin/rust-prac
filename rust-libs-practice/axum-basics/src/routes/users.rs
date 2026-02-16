use axum::{Router, routing::{get,post}, extract::{Path, Query, Json, Form}};
use serde::Deserialize;
use axum::body::Bytes;

#[derive(Deserialize)]
struct UserQuery {
  id: i32,
  name: String,
}

#[derive(Deserialize)]
struct User {
  id: Option<i32>,
  name: Option<String>
}

async fn hello_with_id(Path(id): Path<i32>) -> String {
  format!("Hello, number {}", id)
}

async fn hello_with_query(Query(user_query): Query<UserQuery>) -> String {
  format!("Hello, number {}, user name is {}", user_query.id, user_query.name)
}

async fn hello_with_nickname(nickname: Bytes) -> String {
  format!("Hello, nickname {}", String::from_utf8_lossy(&nickname))
}

// async fn hello_with_nickname(nickname: String) -> String {
//   format!("Hello, nickname {}", nickname)
// }

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

use axum::body::Body;
use futures_util::stream::{self, StreamExt};
use std::{convert::Infallible, time::Duration};
use tokio::time::sleep;

async fn users_stream() -> Body {
    let stream = stream::iter(vec![
        Ok::<Bytes, Infallible>(Bytes::from("user1\n")),
        Ok(Bytes::from("user2\n")),
        Ok(Bytes::from("user3\n")),
    ]);
    Body::from_stream(stream)
}

async fn users_stream_delayed() -> Body {
    let stream = stream::iter(["user1\n", "user2\n", "user3\n"])
      .then(|text| async move {
        sleep(Duration::from_secs(1)).await;
        Ok::<Bytes, Infallible>(Bytes::from(text))
    });
  Body::from_stream(stream)
}

pub fn create_router() -> Router {
  let users_routes = Router::new()
    .merge(
      Router::new()
        .route("/{id}", get(hello_with_id))
        .route("/", get(hello_with_query))
        .route("/", post(hello_with_nickname))
        .route("/stream",get(users_stream))
        .route("/stream/delayed",get(users_stream_delayed))
    )
    .nest(
      "/greet",
      Router::new()
        .route("/id/{id}", get(greet_with_id))
        .route("/name/{name}", get(greet_with_name))
        .route("/{id}/{name}", get(greet))
        .route("/id", get(greet_with_id_query))
        .route("/name", get(greet_with_name_query))
        .route("/", get(greet_with_query))
    )
    .nest(
      "/info",
      Router::new()
          .route("/id", post(user_id_info))
          .route("/name", post(user_name_info))
          .route("/json", post(user_info_json))
          .route("/form", post(user_info_form))
    );

  Router::new()
    .nest("/users", users_routes)
}