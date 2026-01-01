mod common;
use common::setup_test_db;
use SQLite_based_Rust_server::db::queries::insert_item;

#[tokio::test]
async fn insert_item_success() {
    let pool = setup_test_db().await;

    let item = insert_item(&pool, "Milk").await.unwrap();

    assert_eq!(item.name, "Milk");
}