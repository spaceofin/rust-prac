mod entities;

use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| sea_orm::DbErr::Custom("DATABASE_URL must be set".into()))?;

    let connection = Database::connect(database_url).await?;
    println!("Database Connected!");

    println!("Running migrations...");
    Migrator::up(&connection, None).await?;
    println!("Done");

    Ok(())
}
