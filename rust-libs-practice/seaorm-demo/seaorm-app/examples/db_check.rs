use sea_orm::EntityTrait;
use seaorm_app::entities::user;
use seaorm_app::establish_connection;

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    let users: Vec<user::Model> = user::Entity::find().all(&conn).await.unwrap();

    println!("users:\n{:?}", users);
}
