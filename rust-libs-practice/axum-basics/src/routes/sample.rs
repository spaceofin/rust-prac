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

async fn get_foo_baz() -> &'static str {
    "GET /foo/baz"
}

pub fn create_router() -> Router {

  let foo_routes = Router::new()
      .route(
        "/",
          get(get_foo)
            .post(post_foo))
      .nest(
        "/bar",
        Router::new()
          .route("/", get(get_foo_bar)))
      .nest(
        "/baz",
        Router::new()
          .route("/", get(get_foo_baz)));

  Router::new()
    .route(
      "/",
      get(|| async { "Hello, World!" })
        .post(|| async {"Post"})
        .put(|| async {"Put"})
        .delete(|| async {"Delete"})
      )
    .nest("/foo", foo_routes)
}