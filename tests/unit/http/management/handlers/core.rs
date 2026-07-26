use axum::Json;
use axum::extract::State;
use limina::http::management::handlers::{health, users, version};
use limina::state::AppState;

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let response = health::health(State(AppState::default())).await;

    assert_eq!(response, "ok");
}

#[tokio::test]
async fn health_live_returns_ok() {
    let response = health::live(State(AppState::default())).await;

    assert_eq!(response, "ok");
}

#[tokio::test]
async fn health_ready_returns_ready_true() {
    let Json(response) = health::ready(State(AppState::default())).await;

    assert!(response.ready);
}

#[tokio::test]
async fn users_endpoint_returns_empty_array_json() {
    let response = users::users().await;

    assert_eq!(response, "[]");
}

#[tokio::test]
async fn version_endpoint_returns_package_version() {
    let response = version::version(State(AppState::default())).await;

    assert_eq!(response, env!("CARGO_PKG_VERSION"));
}