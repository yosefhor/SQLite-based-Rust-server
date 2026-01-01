use axum::Router;
use sqlx::SqlitePool;
use crate::{AppState, api::router::create_router};

pub async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    pool
}

pub async fn test_app() -> Router {
    let pool = setup_test_db().await;
    let state = AppState { db: pool };
    create_router(state)
}
