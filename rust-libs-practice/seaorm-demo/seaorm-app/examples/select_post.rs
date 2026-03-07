use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ModelTrait};
use seaorm_app::entities::{posts, users};
use seaorm_app::establish_connection;

async fn select_related_posts_lazy(
    conn: &DatabaseConnection,
    user_id: i64,
) -> Result<Vec<posts::Model>, DbErr> {
    let user = users::Entity::find_by_id(user_id).one(conn).await?.unwrap();
    let posts = user.find_related(posts::Entity).all(conn).await?;

    Ok(posts)
}

async fn select_related_posts_eager_one(
    conn: &DatabaseConnection,
) -> Result<Vec<(posts::Model, Option<users::Model>)>, DbErr> {
    let posts_with_users = posts::Entity::find()
        .find_also_related(users::Entity)
        .all(conn)
        .await?;

    Ok(posts_with_users)
}

async fn select_related_posts_eager_many(
    conn: &DatabaseConnection,
) -> Result<Vec<(users::Model, Vec<posts::Model>)>, DbErr> {
    let users_with_posts = users::Entity::find()
        .find_with_related(posts::Entity)
        .all(conn)
        .await?;

    Ok(users_with_posts)
}

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    // let related_posts = select_related_posts_lazy(&conn, 1).await;
    // match related_posts {
    //     Ok(posts) => println!("Selected posts:\n{:#?}", posts),
    //     Err(e) => eprintln!("Select failed:\n{:?}", e),
    // }

    // let related_posts_with_users = select_related_posts_eager_one(&conn).await;
    // match related_posts_with_users {
    //     Ok(posts) => println!("Selected posts with users:\n{:#?}", posts),
    //     Err(e) => eprintln!("Select failed:\n{:?}", e),
    // }

    let related_users_with_posts = select_related_posts_eager_many(&conn).await;
    match related_users_with_posts {
        Ok(users) => println!("Selected users with posts:\n{:#?}", users),
        Err(e) => eprintln!("Select failed:\n{:?}", e),
    }
}
