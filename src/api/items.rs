use crate::error::AppError;
use crate::models::item::Item;
use crate::services::item_service::{self, ServiceError};
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
    let name = payload.name.trim();
    let item = match item_service::create_item(&state.db, &name).await {
        Ok(item) => item,
        Err(ServiceError::InvalidName) => {
            tracing::warn!(
                name = %payload.name,
                "Invalid item name"
            );
            return Err(AppError::BadRequest("name cannot be empty".into()));
        }
        Err(e) => return Err(e.into()),
    };
    Ok(Json(item))
}

pub async fn list_items(State(state): State<AppState>) -> AppResult<Json<Vec<Item>>> {
    let items = item_service::get_items(&state.db).await?;
    Ok(Json(items))
}
