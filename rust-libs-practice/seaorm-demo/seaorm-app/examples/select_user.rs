use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder};
use seaorm_app::entities::users;
use seaorm_app::establish_connection;

async fn select_first_user(conn: &DatabaseConnection) -> Result<users::Model, DbErr> {
    let user = users::Entity::find()
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("No user found".to_owned()))?;

    Ok(user)
}

async fn select_user_by_id(conn: &DatabaseConnection, id: i64) -> Result<users::Model, DbErr> {
    let user = users::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("No user found".to_owned()))?;

    Ok(user)
}

async fn select_users_by_filter(conn: &DatabaseConnection) -> Result<Vec<users::Model>, DbErr> {
    let users: Vec<users::Model> = users::Entity::find()
        .filter(users::COLUMN.username.contains("o"))
        .order_by_asc(users::COLUMN.username)
        .all(conn)
        .await?;

    Ok(users)
}

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    // let selected_user = select_first_user(&conn).await;
    // match selected_user {
    //     Ok(user) => println!("Selected user:\n{:?}", user),
    //     Err(e) => eprintln!("Select failed:\n{:?}", e),
    // }

    // let selected_users = select_users_by_filter(&conn).await;
    // match selected_users {
    //     Ok(users) => println!("Selected users:\n{:#?}", users),
    //     Err(e) => eprintln!("Select failed:\n{:?}", e),
    // }

    let selected_user = select_user_by_id(&conn, 3).await;
    match selected_user {
        Ok(user) => println!("Selected user:\n{:?}", user),
        Err(e) => eprintln!("Select failed:\n{:?}", e),
    }
}
