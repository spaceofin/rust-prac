pub mod entities;
pub mod handlers;
pub mod routes;
pub mod state;

use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").map_err(|_| DbErr::Custom("DATABASE_URL must be set".into()))?;

    let connection = Database::connect(database_url).await?;
    println!("Database Connected!");

    println!("Running migrations...");
    Migrator::up(&connection, None).await?;
    println!("Done");

    Ok(connection)
}
