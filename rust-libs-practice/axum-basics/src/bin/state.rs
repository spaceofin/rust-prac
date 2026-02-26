use axum::{
	Router, 
	extract::{FromRef, State}, 
	routing::get
};
use std::sync::{Arc, Mutex};

// Newtype to avoid String type collision with app_name in State extraction.
#[derive(Clone)]
struct AuthToken(String);

#[derive(FromRef, Clone)]
struct AppState {
	app_name: String,
	version: u32,
	auth_token: AuthToken,
}

async fn get_app_name(State(app_name): State<String>) -> String {
	app_name
}

async fn get_version(State(version): State<u32>) -> String {
	version.to_string()
}

async fn get_auth_token(State(token): State<AuthToken>) -> String {
	token.0
}

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


fn create_data_router() -> Router {
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

fn create_app_info_router() -> Router {
	let state = AppState {
		app_name: "my-app".to_string(),
		version: 1,
		auth_token: AuthToken("abc123".to_string())
	};

	Router::new()
		.route("/app-name", get(get_app_name))
		.route("/version", get(get_version))
		.route("/auth-token", get(get_auth_token))
		.with_state(state)
}

#[tokio::main]
async fn main() {
	// let app = create_data_router();
	let app = create_app_info_router();
	
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
