use axum::extract::State;
use limina::http::management::handlers::metrics;
use limina::state::AppState;

#[tokio::test]
async fn metrics_returns_prometheus_text() {
    let response = metrics::metrics(State(AppState::default())).await;

    assert!(response.contains("# HELP limina_up Gateway process health"));
    assert!(response.contains("limina_up 1"));
}