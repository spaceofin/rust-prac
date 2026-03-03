use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use seaorm_app::entities::users;
use seaorm_app::establish_connection;

async fn select_first_user(conn: &DatabaseConnection) -> Result<users::Model, DbErr> {
    let user = users::Entity::find()
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("No user found".to_owned()))?;

    Ok(user)
}

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    let selected_user = select_first_user(&conn).await;
    match selected_user {
        Ok(user) => println!("Selected user:\n{:?}", user),
        Err(e) => eprintln!("Select failed:\n{:?}", e),
    }
}
