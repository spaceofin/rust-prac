use axum::{extract::State, routing::get, Router};
use std::sync::{Arc, Mutex};


async fn update_first_string(State(data): State<Arc<Mutex<Vec<String>>>>) -> String {
	let mut data = data.lock().unwrap();
	data[0].push('z');
	format!("{:?}", data)
}

async fn update_second_string(State(data): State<Arc<Mutex<Vec<String>>>>) -> String {
	let mut data = data.lock().unwrap();
	data[1].push('z');
	format!("{:?}", data)
}


fn create_router() -> Router {
	let v = vec!["aa", "bb", "cc"]
    	.into_iter()
    	.map(String::from)
    	.collect::<Vec<String>>();

	let data = Arc::new(Mutex::new(v));
	
	Router::new()
		.route("/first", get(update_first_string))
		.route("/second", get(update_second_string))
		.with_state(data)
}

#[tokio::main]
async fn main() {
	let app = create_router();
	
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
