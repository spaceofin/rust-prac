use crate::handlers::{health, users};
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .nest("/users", users::router())
        .with_state(state)
}
