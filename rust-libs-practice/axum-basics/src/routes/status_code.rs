use axum::{
    Router,
    routing::get,
    http::StatusCode,
    Json,
    extract::Path,
    response::{IntoResponse, Response},
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

enum SomeOk {
    Ok,         // 200
    Created,    // 201
    NoContent,  // 204
}

enum SomeError {
    BadRequest,           // 400
    Unauthorized,         // 401
    Forbidden,            // 403
    NotFound,             // 404
    Conflict,             // 409
    TooManyRequests,      // 429
    InternalServerError,  // 500
}

async fn check_something(code: u32) -> Result<SomeOk, SomeError> {
    match code {
        200 => Ok(SomeOk::Ok),
        201 => Ok(SomeOk::Created),
        204 => Ok(SomeOk::NoContent),
        400 => Err(SomeError::BadRequest),
        401 => Err(SomeError::Unauthorized),
        403 => Err(SomeError::Forbidden),
        404 => Err(SomeError::NotFound),
        409 => Err(SomeError::Conflict),
        429 => Err(SomeError::TooManyRequests),
        500 => Err(SomeError::InternalServerError),
        _ => Err(SomeError::InternalServerError)
    }
}

async fn message_with_status_result(Path(code): Path<u32>) -> Response {
    match check_something(code).await {
        Ok(success) => match success {
            SomeOk::Ok => (StatusCode::OK, "Ok!").into_response(),
            SomeOk::Created => (StatusCode::CREATED, "Created!").into_response(),
            // 204 responses must not include a message body
            SomeOk::NoContent => (StatusCode::NO_CONTENT).into_response(),
        },
        Err(e) => match e {
            SomeError::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request...").into_response(),
            SomeError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized...").into_response(),
            SomeError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden...").into_response(),
            SomeError::NotFound => (StatusCode::NOT_FOUND, "Not Found...").into_response(),
            SomeError::Conflict => (StatusCode::CONFLICT, "Conflict...").into_response(),
            SomeError::TooManyRequests => (StatusCode::TOO_MANY_REQUESTS, "Too Many Requests...").into_response(),
            SomeError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error...").into_response(),
        },
    }
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
                .route("/check/{code}", get(message_with_status_result))
        )
}