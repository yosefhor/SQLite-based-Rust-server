use axum::{Json, Router, routing::{get, post}};
use crate::AppState;

use super::items;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { Json(serde_json::json!({"status": "ok"})) }))
        .route("/items", post(items::create_item))
        .route("/items", get(items::list_items))
        .with_state(state)
}