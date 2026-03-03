use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};
use seaorm_app::entities::users;
use seaorm_app::establish_connection;

async fn update_user(
    conn: &DatabaseConnection,
    id: i32,
    username: &str,
) -> Result<users::Model, DbErr> {
    let mut user: users::ActiveModel = users::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("User not found".to_owned()))?
        .into();

    user.username = Set(username.to_owned());
    let user = user.update(conn).await?;

    Ok(user)
}

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    let updated_user = update_user(&conn, 1, "sophia").await;
    match updated_user {
        Ok(user) => println!("Updated user:\n{:?}", user),
        Err(e) => eprintln!("Update failed:\n{:?}", e),
    }
}
