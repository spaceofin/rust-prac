use axum::{Router, routing::get, extract::{Path, Query}};
use std::collections::HashMap;

async fn greet_with_id(Path(id): Path<u32>) -> String {
  format!("Welcome, number {}", id)
}

async fn greet_with_name(Path(name): Path<String>) -> String {
  format!("Welcome, {}", name)
}

async fn greet(Path((id,name)): Path<(u32, String)>) -> String {
  format!("Welcome, number {} {}", id, name)
}

async fn greet_with_id_query(Query(user): Query<HashMap<String, String>>) -> String {
  format!("Welcome, number {}", user["id"].parse::<u32>().unwrap())
}

async fn greet_with_name_query(Query(user): Query<HashMap<String, String>>) -> String {
  format!("Welcome, {}", user["name"])
}

async fn greet_with_query(Query(user): Query<HashMap<String, String>>) -> String {
  format!("Welcome, number {} {}", user["id"].parse::<u32>().unwrap(), user["name"])
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
}