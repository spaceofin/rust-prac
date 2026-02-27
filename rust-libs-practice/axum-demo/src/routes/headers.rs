use axum::{Router, routing::get};
use axum::http::header::HeaderMap;

async fn get_headers(headers: HeaderMap) -> String {
  use axum::http::header::{ HOST, USER_AGENT, ACCEPT, ACCEPT_LANGUAGE };

  let host= headers
    .get(HOST)
    .map(|v| v.to_str().unwrap().to_string());

  let user_agent = headers
    .get(USER_AGENT)
    .map(|v| v.to_str().unwrap().to_string());

  let accept = headers
    .get(ACCEPT)
    .map(|v| v.to_str().unwrap().to_string());

  let accept_language = headers
    .get(ACCEPT_LANGUAGE)
    .map(|v| v.to_str().unwrap().to_string());

	format!(
		"Host: {},\n\
        User-Agent: {},\n\
        Accept: {},\n\
        Accept-Language: {}",
        host.unwrap_or_default(),
        user_agent.unwrap_or_default(),
        accept.unwrap_or_default(),
        accept_language.unwrap_or_default(),
	)
}

async fn post_headers(headers: HeaderMap) -> String {
   use axum::http::header::{CONTENT_TYPE, CONTENT_LENGTH};

  let content_type = headers
    .get(CONTENT_TYPE)
    .map(|v| v.to_str().unwrap().to_string());

  let content_length = headers
    .get(CONTENT_LENGTH)
    .map(|v| v.to_str().unwrap().to_string());

	format!(
        "Content-Type: {},\n\
        Content-Length: {}",
        content_type.unwrap_or_default(),
        content_length.unwrap_or_default(),
	)
}

use axum_extra::{
  extract::TypedHeader,
  headers::{ Host, UserAgent, ContentType, ContentLength}
};

async fn get_typed_headers(
  TypedHeader(host): TypedHeader<Host>,
  TypedHeader(user_agent): TypedHeader<UserAgent>,) -> String {
	format!(
		"Host: {},\n\
    User-Agent: {}",
    host,
    user_agent
	)
}

async fn post_typed_headers(
  TypedHeader(content_type): TypedHeader<ContentType>,
  TypedHeader(content_length): TypedHeader<ContentLength>,
) -> String {
	format!(
    "Content-Type: {},\n\
    Content-Length: {}",
    content_type,
    content_length.0,
	)
}

pub fn create_router() -> Router {
    Router::new()
        .route(
            "/headers",
            get(get_headers)
            .post(post_headers),
        )
        .route(
            "/typed-headers",
            get(get_typed_headers)
            .post(post_typed_headers),
        )
}