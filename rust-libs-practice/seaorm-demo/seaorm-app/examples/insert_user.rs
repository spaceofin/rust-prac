use sea_orm::DbErr;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use seaorm_app::entities::users;
use seaorm_app::establish_connection;

async fn insert_user(conn: &DatabaseConnection, username: &str) -> Result<users::Model, DbErr> {
    let user = users::ActiveModel {
        username: Set(username.to_string()),
        ..Default::default()
    }
    .insert(conn)
    .await?;

    Ok(user)
}

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    let inserted_user = insert_user(&conn, "alice").await;
    match inserted_user {
        Ok(user) => println!("Inserted user:\n{:?}", user),
        Err(e) => eprintln!("Insert failed:\n{:?}", e),
    }
}
