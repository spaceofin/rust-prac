use std::{
    collections::HashMap,
    time::Duration,
    sync::{Arc, Mutex}
};
use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    routing::post,
    Json,
    Router
};
use serde::{Serialize, Deserialize};

type Cache = Arc<Mutex<HashMap<u64, Bytes>>>;

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
	if let Some(body) = state.lock().unwrap().get(&product.id).cloned() {
		println!("Cache Hit:\n{:?}", product);
		return (StatusCode::OK, body);
	}
	
	println!("Cache Miss:\n{:?}", product);

    tokio::time::sleep(Duration::from_millis(3000)).await;

    let body = Bytes::from(serde_json::to_vec(&product).unwrap());
	let mut cache = state.lock().unwrap();
	cache.insert(product.id, body.clone());
	
	(StatusCode::OK, body)
}

#[tokio::main]
async fn main() {
	let state: Cache = Arc::new(Mutex::new(HashMap::new()));
	let app = Router::new()
        .nest("/proxy",
            Router::new()
        		.route("/product", post(proxy_product))
		        .with_state(state)
        );
		
	let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
		.await
		.unwrap();
	axum::serve(listener, app).await.unwrap();
}