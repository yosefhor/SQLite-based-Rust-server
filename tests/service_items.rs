use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use crate::tests::common::setup_test_db;

#[tokio::test]
async fn create_item_empty_name_returns_error() {
    let pool = setup_test_db().await;

    let res = create_item(&pool, "".to_string()).await;

    assert!(matches!(res, Err(ServiceError::InvalidName)));
}
