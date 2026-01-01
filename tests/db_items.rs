use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use crate::tests::common::setup_test_db;

#[tokio::test]
async fn insert_item_success() {
    let pool = setup_test_db().await;

    let item = insert_item(&pool, "Milk").await.unwrap();

    assert_eq!(item.name, "Milk");
}
