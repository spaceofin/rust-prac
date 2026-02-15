mod routes;

// use routes::sample_routes;
use routes::users_routes;
// use routes::files_routes;

#[tokio::main]
async fn main() {
    // let app = files_routes::create_router();
    let app = users_routes::create_router();
    // let app = sample_routes::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}