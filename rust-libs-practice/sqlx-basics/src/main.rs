use std::error::Error;
use dotenv::dotenv;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let postgres_pool = db::create_postgres_pool().await?;
    let mysql_pool = db::create_mysql_pool().await?;

    db::init_postgres_db(&postgres_pool).await?;
    db::init_mysql_db(&mysql_pool).await?;

    Ok(())
}
