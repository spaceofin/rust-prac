use crate::handlers::health;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .with_state(state)
}
