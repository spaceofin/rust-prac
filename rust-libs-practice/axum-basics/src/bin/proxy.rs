use std::{
    collections::HashMap,
    time::Duration,
    sync::{Arc, Mutex}
};
use axum::{
    body::Bytes,
    extract::State,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json,
    Router
};
use serde::{Serialize, Deserialize};

#[derive(Hash, Eq, PartialEq)]
enum CacheKey {
    Product(u64),
    Post(u64),
}

type Cache = Arc<Mutex<HashMap<CacheKey, Bytes>>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    id: u64,
    name: String,
    price: u32,
    in_stock: bool,
}

async fn proxy_product(
	State(state): State<Cache>,
	Json(product): Json<Product>,
) -> (StatusCode, Bytes) {
    let key = CacheKey::Product(product.id);
	if let Some(body) = state.lock().unwrap().get(&key).cloned() {
		println!("Cache Hit:\n{:?}", product);
		return (StatusCode::OK, body);
	}
	
	println!("Cache Miss:\n{:?}", product);

    tokio::time::sleep(Duration::from_millis(3000)).await;

    let body = Bytes::from(serde_json::to_vec(&product).unwrap());
	let mut cache = state.lock().unwrap();
	cache.insert(key, body.clone());
	
	(StatusCode::OK, body)
}

use reqwest::Client;

const BASE_POST_URL: &str = "https://dummyjson.com/posts";

async fn proxy_post(
	State(state): State<Cache>,
	Path(post_id): Path<u64>,
) -> (StatusCode, Bytes) {
    let key = CacheKey::Post(post_id);
	if let Some(body) = state.lock().unwrap().get(&key).cloned() {
		println!("Cache Hit: Post id {}", post_id);
        println!("Cached body: {:?}", body);
		return (StatusCode::OK, body);
	}
	
	println!("Cache Miss: Post id {}", post_id);

    tokio::time::sleep(Duration::from_millis(3000)).await;

    let url = format!("{}/{}", BASE_POST_URL, post_id);
	let client = Client::new();
	let response = client.get(url).send().await.unwrap();
	let status_code = response.status().as_u16();
	let body = response.bytes().await.unwrap();

	let mut cache = state.lock().unwrap();
	cache.insert(key, body.clone());

	(StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR), body)
}

#[tokio::main]
async fn main() {
	let state: Cache = Arc::new(Mutex::new(HashMap::new()));
	let app = Router::new()
        .nest("/proxy",
            Router::new()
        		.route("/products", post(proxy_product))
                .route("/posts/{id}", get(proxy_post))
		        .with_state(state)
        );
		
	let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
		.await
		.unwrap();
	axum::serve(listener, app).await.unwrap();
}