mod api;
mod config;
mod db;
mod models;
mod services;

use std::net::SocketAddr;
use config::Config;
use sqlx::SqlitePool;
use tracing::{error, info};
use tracing_subscriber::fmt::init;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() {
    init();

    let config = Config::load();

    let pool = SqlitePool::connect(&config.database_url)
        .await
        .expect("DB connection failed");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migration failed");

    let state = AppState { db: pool };

    let app = api::router::create_router(state);

    let addr: SocketAddr = format!("0.0.0.0:{}", config.server_port)
        .parse()
        .unwrap_or_else(|_| {
            error!("Invalid port {}, falling back to 3000", config.server_port);
            "0.0.0.0:3000".parse().unwrap()
        });

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            error!(error = %e, "Failed to bind server");
            return;
        }
    };

    info!("Server running on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
