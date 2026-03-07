use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, ExecResult};
use seaorm_app::establish_connection;

async fn insert_posts(conn: &DatabaseConnection) -> Result<ExecResult, DbErr> {
    let exec_res = conn
        .execute_unprepared(
            r#"
            INSERT INTO posts (id, user_id, title, content) VALUES
            (1, 1, 'First Post', 'This is the first post content'),
            (2, 1, 'Second Post', 'This is the second post content'),
            (3, 2, 'Rust Tips', 'Some useful Rust tips'),
            (4, 2, 'SeaORM Guide', 'Introduction to SeaORM usage'),
            (5, 3, 'SQLite Notes', 'Basic notes about SQLite');
            "#,
        )
        .await?;

    Ok(exec_res)
}

async fn insert_tags(conn: &DatabaseConnection) -> Result<ExecResult, DbErr> {
    let exec_res = conn
        .execute_unprepared(
            r#"
            INSERT INTO tags (id, name) VALUES
            (1, 'rust'),
            (2, 'seaorm'),
            (3, 'sqlite');
            "#,
        )
        .await?;

    Ok(exec_res)
}

async fn insert_post_tags(conn: &DatabaseConnection) -> Result<ExecResult, DbErr> {
    let exec_res = conn
        .execute_unprepared(
            r#"
            INSERT INTO post_tags (post_id, tag_id) VALUES
            (3, 1),
            (4, 2),
            (5, 3);
            "#,
        )
        .await?;

    Ok(exec_res)
}

#[tokio::main]
async fn main() {
    let conn = establish_connection().await.unwrap();

    // let insert_posts_result = insert_posts(&conn).await;
    // match insert_posts_result {
    //     Ok(res) => println!("Insert result:\n{:?}", res),
    //     Err(e) => eprintln!("Insert failed:\n{:?}", e),
    // }

    let insert_tags_result = insert_tags(&conn).await;
    match insert_tags_result {
        Ok(res) => println!("Insert result:\n{:?}", res),
        Err(e) => eprintln!("Insert failed:\n{:?}", e),
    }

    let insert_post_tags_result = insert_post_tags(&conn).await;
    match insert_post_tags_result {
        Ok(res) => println!("Insert result:\n{:?}", res),
        Err(e) => eprintln!("Insert failed:\n{:?}", e),
    }
}
