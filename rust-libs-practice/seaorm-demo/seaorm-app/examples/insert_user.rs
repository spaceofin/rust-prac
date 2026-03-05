use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use sea_orm::{DbErr, InsertManyResult};
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

async fn insert_users(
    conn: &DatabaseConnection,
    usernames: Vec<&str>,
) -> Result<InsertManyResult<users::ActiveModel>, DbErr> {
    let active_models: Vec<users::ActiveModel> = usernames
        .into_iter()
        .map(|username| users::ActiveModel {
            username: Set(username.to_string()),
            ..Default::default()
        })
        .collect();

    let res = users::Entity::insert_many(active_models).exec(conn).await?;

    Ok(res)
}

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    // let inserted_user = insert_user(&conn, "alice").await;
    // match inserted_user {
    //     Ok(user) => println!("Inserted user:\n{:?}", user),
    //     Err(e) => eprintln!("Insert failed:\n{:?}", e),
    // }

    let inserted_users_result = insert_users(&conn, vec!["mary", "robert", "david"]).await;
    match inserted_users_result {
        Ok(res) => println!("Insert result:\n{:?}", res),
        Err(e) => eprintln!("Insert failed:\n{:?}", e),
    }
}
