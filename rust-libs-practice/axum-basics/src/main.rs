mod routes;

// use routes::sample_routes;
// use routes::user_routes;
use routes::files_routes;

#[tokio::main]
async fn main() {
    let app = files_routes::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}