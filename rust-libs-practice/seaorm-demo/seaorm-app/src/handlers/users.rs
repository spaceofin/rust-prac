use crate::entities::users;
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel};
use serde::Deserialize;

type ApiResult<T> = Result<T, (StatusCode, String)>;

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    username: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUserRequest {
    username: String,
}

fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

async fn list_users(State(state): State<AppState>) -> ApiResult<Json<Vec<users::Model>>> {
    let users = users::Entity::find()
        .all(&state.db)
        .await
        .map_err(internal_error)?;
    Ok(Json(users))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<Json<users::Model>> {
    let user = users::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("user not found: {id}")))?;
    Ok(Json(user))
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> ApiResult<(StatusCode, Json<users::Model>)> {
    let inserted = users::ActiveModel {
        username: Set(payload.username),
        ..Default::default()
    }
    .insert(&state.db)
    .await
    .map_err(internal_error)?;
    Ok((StatusCode::CREATED, Json(inserted)))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUserRequest>,
) -> ApiResult<Json<users::Model>> {
    let user = users::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("user not found: {id}")))?;
    let mut active = user.into_active_model();
    active.username = Set(payload.username);
    let updated = active.update(&state.db).await.map_err(internal_error)?;
    Ok(Json(updated))
}

async fn delete_user(State(state): State<AppState>, Path(id): Path<i64>) -> ApiResult<StatusCode> {
    let result = users::Entity::delete_by_id(id)
        .exec(&state.db)
        .await
        .map_err(internal_error)?;
    if result.rows_affected == 0 {
        return Err((StatusCode::NOT_FOUND, format!("user not found: {id}")));
    }
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}
