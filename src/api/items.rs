use axum::{Json, extract::State};
use serde::Deserialize;
use crate::services::item_service;
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItemRequest>,
) -> Json<serde_json::Value> {

    match item_service::create_item(&state.db, payload.name).await {
        Ok(item) => Json(serde_json::json!(item)),
        Err(_) => Json(serde_json::json!({ "error": "invalid input" })),
    }
}

pub async fn list_items(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {

    match item_service::get_items(&state.db).await {
        Ok(items) => Json(serde_json::json!(items)),
        Err(_) => Json(serde_json::json!({ "error": "db error" })),
    }
}
