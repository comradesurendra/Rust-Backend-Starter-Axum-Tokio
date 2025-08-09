use axum::{routing::get, Router};
use tower::util::ServiceExt; // for `oneshot`

#[tokio::test]
async fn health_check_works() {
    let app = Router::new().route("/health", get(|| async { axum::http::StatusCode::OK }));
    let response = app
        .oneshot(axum::http::Request::builder().uri("/health").body(axum::body::Body::empty()).unwrap())
        .await
        .unwrap();
    assert!(response.status().is_success());
}


