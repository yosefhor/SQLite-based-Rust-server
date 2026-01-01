pub mod api;
pub mod config;
pub mod db;
pub mod error;
pub mod logging;
pub mod models;
pub mod server;
pub mod services;

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}