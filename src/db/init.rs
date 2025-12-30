use sqlx::{Pool, Sqlite, SqlitePool};
use std::{fs::File, path::Path};

pub async fn init_db(database_url: &str) -> anyhow::Result<Pool<Sqlite>> {
    let db_path = database_url
        .strip_prefix("sqlite://")
        .unwrap_or(database_url);

    if let Some(parent) = Path::new(db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    if !Path::new(db_path).exists() {
        File::create(db_path)?;
    }

    let pool = SqlitePool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
