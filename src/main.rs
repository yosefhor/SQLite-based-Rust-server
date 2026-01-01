use anyhow::Context;
use std::{env, net::SocketAddr};
use tracing::info;

use SQLite_based_Rust_server::{
    api,
    config,
    db,
    logging,
    server,
    AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "debug".to_string());
    let _log_guard = logging::init_logging(&log_level).context("failed to init logging")?;

    info!("Logging initialized at {} level", log_level);

    let config = config::Config::load();

    let db_pool = db::init::init_db(&config.database_url)
        .await
        .context("failed to init database")?;

    let state = AppState { db: db_pool };
    let app = api::router::create_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));

    let listener = server::bind_with_fallback(addr).await?;

    info!("Server listening on port {}", listener.local_addr()?.port());

    axum::serve(listener, app).await?;

    Ok(())
}
