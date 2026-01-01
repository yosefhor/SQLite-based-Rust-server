use crate::models::item::Item;
use crate::services::item_service;
use crate::{AppState, error::AppResult};
use axum::{Json, extract::State};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItemRequest>,
) -> AppResult<Json<Item>> {
    let item = item_service::create_item(&state.db, payload.name).await?;
    Ok(Json(item))
}

pub async fn list_items(State(state): State<AppState>) -> AppResult<Json<Vec<Item>>> {
    let items = item_service::get_items(&state.db).await?;
    Ok(Json(items))
}
