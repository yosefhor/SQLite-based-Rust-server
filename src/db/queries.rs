use sqlx::{SqlitePool};
use crate::models::item::Item;

pub async fn health_check(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}

pub async fn insert_item(pool: &SqlitePool, name: &str) -> Result<Item, sqlx::Error> {
    let item = sqlx::query_as::<_, Item>(
        r#"
        INSERT INTO items (name, created_at)
        VALUES (?, datetime('now'))
        RETURNING id, name, created_at
        "#
    )
    .bind(name)
    .fetch_one(pool)
    .await?;

    Ok(item)
}

pub async fn list_items(pool: &SqlitePool) -> Result<Vec<Item>, sqlx::Error> {
    let items = sqlx::query_as::<_, Item>(
        "SELECT id, name, created_at FROM items"
    )
    .fetch_all(pool)
    .await?;

    Ok(items)
}
