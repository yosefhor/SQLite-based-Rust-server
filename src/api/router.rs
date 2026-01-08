use crate::AppState;
use axum::{
    Json, Router,
    routing::{get, post},
};
use tower_http::cors::{AllowOrigin, CorsLayer};

use super::items;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap()))
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);
    Router::new()
        .route(
            "/health",
            get(|| async { Json(serde_json::json!({"status": "ok"})) }),
        )
        .route("/items", post(items::create_item))
        .route("/items", get(items::list_items))
        .layer(cors)
        .with_state(state)
}
