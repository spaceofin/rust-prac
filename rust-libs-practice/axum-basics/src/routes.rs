use axum::{Router, routing::get};

async fn get_foo() -> &'static str {
  "GET /foo"
}
async fn post_foo() -> &'static str {
  "POST /foo"
}
async fn get_foo_bar() -> &'static str {
    "GET /foo/bar"
}

pub fn create_router() -> Router {
  Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(get_foo_bar))
}