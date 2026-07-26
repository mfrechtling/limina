use axum::Json;
use axum::extract::State;
use limina::http::management::handlers::routes;
use limina::state::AppState;

#[tokio::test]
async fn routes_returns_management_routes() {
    let Json(response) = routes::routes(State(AppState::default())).await;

    assert!(response.routes.contains(&"/_limina/health/live".to_owned()));
    assert!(
        response
            .routes
            .contains(&"/_limina/config/reload".to_owned())
    );
}
