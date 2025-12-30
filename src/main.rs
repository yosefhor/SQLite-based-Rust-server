mod api;
mod config;
mod db;
mod error;
mod logging;
mod models;
mod services;

use crate::error::{AppError, AppResult};
use sqlx::{SqlitePool, any};
use std::{env, net::SocketAddr};
use tracing::{error, info};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenvy::dotenv().ok();
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "debug".to_string());
    let _log_guard =
        logging::init_logging(&log_level).map_err(|e| AppError::Internal { source: e.into() })?;
    info!("Logging initialized at {} level", log_level);

    let config = config::Config::load();

    let db_pool = db::init::init_db(&config.database_url).await.unwrap();

    let state = AppState { db: db_pool };

    let app = api::router::create_router(state);

    let addr: SocketAddr = format!("0.0.0.0:{}", config.server_port)
        .parse()
        .unwrap_or_else(|_| {
            error!("Invalid port {}, falling back to 3000", config.server_port);
            "0.0.0.0:3000".parse().unwrap()
        });

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to bind server");
            e
        })
        .unwrap();

    info!("Server running on {}", addr);

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
