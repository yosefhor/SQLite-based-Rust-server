use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}
