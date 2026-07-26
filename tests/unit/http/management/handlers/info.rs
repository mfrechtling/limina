use axum::Json;
use axum::extract::State;
use limina::http::management::handlers::info;
use limina::state::AppState;

#[tokio::test]
async fn info_returns_runtime_identity() {
    let Json(response) = info::info(State(AppState::default())).await;

    assert_eq!(response.name, "limina");
    assert_eq!(response.environment, "development");
    assert_eq!(response.instance_id, "limina-local");
    assert!(response.uptime_seconds >= 0);
}