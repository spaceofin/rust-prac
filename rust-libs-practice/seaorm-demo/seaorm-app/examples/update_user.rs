use sea_orm::sqlx::types::chrono;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};
use sea_orm::{ColumnTrait, UpdateResult};
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

async fn update_users_created_at(
    conn: &DatabaseConnection,
    ids: Vec<i32>,
) -> Result<UpdateResult, DbErr> {
    let update_data = users::ActiveModel {
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    let res = users::Entity::update_many()
        .set(update_data)
        .filter(users::Column::Id.is_in(ids))
        .exec(conn)
        .await?;

    Ok(res)
}

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    // let updated_user = update_user(&conn, 1, "sophia").await;
    // match updated_user {
    //     Ok(user) => println!("Updated user:\n{:?}", user),
    //     Err(e) => eprintln!("Update failed:\n{:?}", e),
    // }

    let updated_users_result = update_users_created_at(&conn, vec![4, 5]).await;
    match updated_users_result {
        Ok(res) => println!("Update result:\n{:?}", res),
        Err(e) => eprintln!("Update failed:\n{:?}", e),
    }
}
