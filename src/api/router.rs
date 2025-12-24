use axum::{Router, routing::{get, post}};
use crate::AppState;

use super::items;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/items", post(items::create_item))
        .route("/items", get(items::list_items))
        .with_state(state)
}
