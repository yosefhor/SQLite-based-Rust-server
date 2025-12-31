use crate::error::{AppError, AppResult};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{error, warn};

pub async fn bind_with_fallback(addr: SocketAddr) -> AppResult<TcpListener> {
    match TcpListener::bind(addr).await {
        Ok(listener) => Ok(listener),
        Err(e) => {
            warn!("Failed to bind to {}: {}. Falling back to port 0.", addr, e);
            let fallback_addr = SocketAddr::from(([0, 0, 0, 0], 0));
            let listener = TcpListener::bind(fallback_addr).await.map_err(|e| {
                error!(
                    "Failed to bind to fallback address {}: {}",
                    fallback_addr, e
                );
                AppError::Internal {
                    source: anyhow::anyhow!(e),
                }
            })?;
            Ok(listener)
        }
    }
}
