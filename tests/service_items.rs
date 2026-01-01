mod common;
use common::setup_test_db;
use SQLite_based_Rust_server::services::item_service::{create_item, ServiceError};

#[tokio::test]
async fn create_item_invalid_name() {
    let pool = setup_test_db().await;

    let res = create_item(&pool, "".to_string()).await;

    assert!(matches!(res, Err(ServiceError::InvalidName)));
}

#[tokio::test]
async fn create_item_success() {
    let pool = setup_test_db().await;

    let item = create_item(&pool, "Bread".to_string()).await.unwrap();

    assert_eq!(item.name, "Bread");
}
