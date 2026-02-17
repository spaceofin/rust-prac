mod routes;

// use routes::sample;
// use routes::users;
use routes::files;

#[tokio::main]
async fn main() {
    let app = files::create_router();
    // let app = users::create_router();
    // let app = sample::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}