use sea_orm::{DatabaseConnection, DbErr, DeleteResult, EntityTrait, ModelTrait};
use seaorm_app::entities::users;
use seaorm_app::establish_connection;

async fn delete_user(conn: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
    let res = users::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("User not found".to_owned()))?
        .delete(conn)
        .await?;
    Ok(res)
}

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    let res = delete_user(&conn, 2).await;
    match res {
        Ok(res) => println!("Delete result:\n{:?}", res),
        Err(e) => eprintln!("Delete failed:\n{:?}", e),
    }
}
