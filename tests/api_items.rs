mod common;
use common::test_app;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use serde_json::Value;

#[tokio::test]
async fn get_health() {
    let (_state, app) = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), 1024).await.unwrap(); 
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body["status"], "ok");
}
