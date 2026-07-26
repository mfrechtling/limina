use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use limina::service;
use limina::state::AppState;
use tower::util::ServiceExt;

#[tokio::test]
async fn unknown_route_returns_404() {
    let response = service::http(AppState::default())
        .oneshot(
            Request::builder()
                .uri("/unknown")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("service should return a response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn health_route_returns_ok() {
    let response = service::http(AppState::default())
        .oneshot(
            Request::builder()
                .uri("/_limina/health")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("service should return a response");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn config_validate_rejects_get_requests() {
    let response = service::http(AppState::default())
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/_limina/config/validate")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("service should return a response");

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn config_reload_rejects_get_requests() {
    let response = service::http(AppState::default())
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/_limina/config/reload")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("service should return a response");

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}