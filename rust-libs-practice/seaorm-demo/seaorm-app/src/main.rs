use seaorm_app::{establish_connection, routes::create_router, state::AppState};

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();
    let state = AppState { db: conn };
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
