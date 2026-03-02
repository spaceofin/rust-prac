use seaorm_app::establish_connection;

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();
}
