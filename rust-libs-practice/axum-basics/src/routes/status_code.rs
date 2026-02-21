use axum::{
    Router,
    routing::get,
    http::StatusCode,
    Json
};
use serde::Serialize;
use axum_extra::{
    TypedHeader,
    headers::ContentType
};

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

async fn message_with_ok() -> (StatusCode, Json<ApiResponse>) {
    (
        StatusCode::OK,
        Json(ApiResponse {
            message: "OK".to_string()
        })
    )
}

async fn message_with_created() -> (StatusCode, Json<ApiResponse>) {
    (
        StatusCode::CREATED,
        Json(ApiResponse {
            message: "Created".to_string()
        })
    )
}

async fn message_with_not_found() -> (StatusCode, Json<ApiResponse>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse {
            message: "Not Found".to_string()
        })
    )
}

async fn message_with_typed_header() -> (TypedHeader<ContentType>, Json<ApiResponse>) {
    (
        TypedHeader(ContentType::json()),
        Json(ApiResponse {
                message: "OK".to_string()
        })
    )
}

async fn message_with_status_typed_header() -> (TypedHeader<ContentType>, (StatusCode, Json<ApiResponse>)) {
    (
        TypedHeader(ContentType::json()),
        (
            StatusCode::OK,
            Json(ApiResponse {
                message: "OK".to_string()
            })
        )
    )
}

pub fn create_router() -> Router {
    Router::new()
        .nest(
            "/status-code",
            Router::new()
                .route("/ok", get(message_with_ok))
                .route("/created", get(message_with_created))
                .route("/not-found", get(message_with_not_found))
                .route("/ok-typed-header", get(message_with_typed_header))
                .route("/ok-status-typed-header", get(message_with_status_typed_header))
        )
}