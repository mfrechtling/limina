use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use limina::management::endpoints::config;
use limina::state::{AppState, GatewayConfig};

#[tokio::test]
async fn validate_accepts_supported_mode() {
    let Json(response) = config::validate(
        State(AppState::default()),
        axum::Json(GatewayConfig {
            mode: "normal".to_owned(),
        }),
    )
    .await;

    assert!(response.valid);
    assert!(response.errors.is_empty());
}

#[tokio::test]
async fn validate_rejects_unknown_mode() {
    let Json(response) = config::validate(
        State(AppState::default()),
        axum::Json(GatewayConfig {
            mode: "invalid".to_owned(),
        }),
    )
    .await;

    assert!(!response.valid);
    assert_eq!(
        response.errors,
        vec!["unsupported mode: invalid".to_owned()]
    );
}

#[tokio::test]
async fn reload_updates_state_for_valid_config() {
    let state = AppState::default();

    let (status, Json(response)) = config::reload(
        State(state.clone()),
        axum::Json(GatewayConfig {
            mode: "read-only".to_owned(),
        }),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(response.valid);

    let current_mode = state.current_config.read().await.mode.clone();
    assert_eq!(current_mode, "read-only");
}

#[tokio::test]
async fn reload_returns_bad_request_for_invalid_config() {
    let state = AppState::default();

    let (status, Json(response)) = config::reload(
        State(state.clone()),
        axum::Json(GatewayConfig {
            mode: "invalid".to_owned(),
        }),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(!response.valid);

    let current_mode = state.current_config.read().await.mode.clone();
    assert_eq!(current_mode, "normal");
}
