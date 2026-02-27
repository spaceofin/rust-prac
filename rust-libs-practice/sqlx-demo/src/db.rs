use sqlx::{PgPool, MySqlPool, SqlitePool, Row};
use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;
use std::env;

pub async fn create_postgres_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("POSTGRES_DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}

pub async fn create_mysql_pool() -> Result<MySqlPool, sqlx::Error> {
    let database_url = env::var("MYSQL_DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;
    Ok(pool)
}


pub async fn create_sqlite_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite:data/user.db";
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    Ok(pool)
}

pub async fn init_postgres_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            active BOOLEAN NOT NULL DEFAULT TRUE
        )
    "#;

    sqlx::query(sql).execute(pool).await?;
    println!("Postgres DB initialized");
    Ok(())
}

pub async fn init_mysql_db(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            email VARCHAR(255) NOT NULL UNIQUE,
            active BOOLEAN NOT NULL DEFAULT TRUE
        )
    "#;

    sqlx::query(sql).execute(pool).await?;
    println!("MySQL DB initialized");
    Ok(())
}

pub async fn init_sqlite_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            active BOOLEAN NOT NULL DEFAULT 1
        )
    "#;

    sqlx::query(sql).execute(pool).await?;
    println!("SQLite DB initialized");
    Ok(())
}

pub async fn print_sqlite_db_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("------------------------------SQLite Database Tables------------------------------");

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