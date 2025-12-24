use crate::db::queries;
use crate::models::item::Item;
use sqlx::SqlitePool;

#[derive(Debug)]
pub enum ServiceError {
    InvalidName,
    Database(sqlx::Error),
}

pub async fn create_item(
    pool: &SqlitePool,
    name: String,
) -> Result<Item, ServiceError> {

    if name.trim().is_empty() {
        return Err(ServiceError::InvalidName);
    }

    queries::insert_item(pool, &name)
        .await
        .map_err(ServiceError::Database)
}

pub async fn get_items(
    pool: &SqlitePool,
) -> Result<Vec<Item>, ServiceError> {
    queries::list_items(pool)
        .await
        .map_err(ServiceError::Database)
}
