use sqlx::{SqlitePool, Row};
use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;
use std::error::Error;

pub async fn create_pool() -> Result<SqlitePool, Box<dyn Error>> {
    let options = SqliteConnectOptions::from_str("sqlite:data/user.db")?
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    Ok(pool)
}

pub async fn init_db(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            active BOOLEAN NOT NULL DEFAULT 1
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn print_db_schema(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    println!("------------------------------Database Tables------------------------------");

    let tables = sqlx::query("SELECT name, sql FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")
        .fetch_all(pool)
        .await?;

    for table in &tables {
        let name: String = table.get("name");
        let sql: String = table.get("sql");

        println!("[Table] {}", name);
        println!("[Schema]\n{}\n", sql);

        let query = format!("PRAGMA table_info({})", name);
        let columns = sqlx::query(&query)
            .fetch_all(pool)
            .await?;

        println!("[Columns]");
        for col in columns {
            let cid: i64 = col.get("cid");
            let col_name: String = col.get("name");
            let col_type: String = col.get("type");
            let notnull: i64 = col.get("notnull");
            let dflt_value: Option<String> = col.get("dflt_value");
            let pk: i64 = col.get("pk");

            let dflt_value_str = dflt_value.as_deref().unwrap_or("None");

            println!(
                "  cid: {:<4} name: {:<10} type: {:<10} notnull: {:<4} default: {:<10} pk: {:<4}",
                cid, col_name, col_type, notnull, dflt_value_str, pk
            );
        }
    }
    println!("--------------------------------------------------------------------------------");

    Ok(())
}