use crate::state::AppState;
use axum::Router;

pub fn create_router(state: AppState) -> Router {
    Router::new().with_state(state)
}
