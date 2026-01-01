use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use crate::tests::common::test_app;

#[tokio::test]
async fn post_items_returns_200() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::post("/items")
                .header("content-type", "application/json")
                .body(r#"{ "name": "Milk" }"#.into())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
