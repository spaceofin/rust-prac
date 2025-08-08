use std::error::Error;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = db::create_pool().await?;
    db::init_db(&pool).await?;

    db::print_db_schema(&pool).await?;

    Ok(())
}
