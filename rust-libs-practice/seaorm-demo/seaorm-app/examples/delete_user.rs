use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, ModelTrait, QueryFilter,
};
use seaorm_app::entities::users;
use seaorm_app::establish_connection;

async fn delete_user(conn: &DatabaseConnection, id: i64) -> Result<DeleteResult, DbErr> {
    let res = users::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("User not found".to_owned()))?
        .delete(conn)
        .await?;
    Ok(res)
}

async fn delete_user_direct(conn: &DatabaseConnection, id: i64) -> Result<DeleteResult, DbErr> {
    let res = users::Entity::delete_by_id(id).exec(conn).await?;

    Ok(res)
}

async fn delete_user_returning(
    conn: &DatabaseConnection,
    id: i64,
) -> Result<Option<users::Model>, DbErr> {
    let deleted_user = users::Entity::delete_by_id(id)
        .exec_with_returning(conn)
        .await?;

    Ok(deleted_user)
}

async fn delete_users_by_pattern(
    conn: &DatabaseConnection,
    pattern: &str,
) -> Result<DeleteResult, DbErr> {
    let res = users::Entity::delete_many()
        .filter(users::Column::Username.contains(pattern))
        .exec(conn)
        .await?;

    Ok(res)
}

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    // let res = delete_user(&conn, 2).await;
    // match res {
    //     Ok(res) => println!("Delete result:\n{:?}", res),
    //     Err(e) => eprintln!("Delete failed:\n{:?}", e),
    // }

    // let res = delete_user_direct(&conn, 6).await;
    // match res {
    //     Ok(res) => println!("Delete result:\n{:?}", res),
    //     Err(e) => eprintln!("Delete failed:\n{:?}", e),
    // }

    // let deleted_user = delete_user_returning(&conn, 6).await;
    // match deleted_user {
    //     Ok(user) => println!("Delete user:\n{:?}", user),
    //     Err(e) => eprintln!("Delete failed:\n{:?}", e),
    // }

    let res = delete_users_by_pattern(&conn, "ee").await;
    match res {
        Ok(res) => println!("Delete result:\n{:?}", res),
        Err(e) => eprintln!("Delete failed:\n{:?}", e),
    }
}
