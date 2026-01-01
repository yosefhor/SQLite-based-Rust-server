use axum::{http::StatusCode, response::IntoResponse};

use crate::services::item_service::ServiceError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found")]
    NotFound,

    #[error("Internal server error")]
    Internal {
        #[from]
        source: anyhow::Error,
    },
}

impl From<ServiceError> for AppError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::InvalidName =>
                AppError::BadRequest("Invalid name".into()),

            ServiceError::Database(e) =>
                AppError::Internal { source: e.into() },
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),
            AppError::Internal { source } => {
                tracing::error!(error = %source, "Internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}
