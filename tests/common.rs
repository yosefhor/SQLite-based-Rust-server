use SQLite_based_Rust_server::{db, AppState};
use sqlx::{SqlitePool};

pub async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#
    ).execute(&pool).await.unwrap();

    pool
}

pub async fn test_app() -> (AppState, axum::Router) {
    let pool = setup_test_db().await;
    let state = AppState { db: pool.clone() };

    let app = SQLite_based_Rust_server::api::router::create_router(state.clone());

    (state, app)
}
