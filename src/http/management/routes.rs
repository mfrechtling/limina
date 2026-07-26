use axum::{
    Router,
    routing::{get, post},
};

use crate::http::management::handlers;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/_limina/health", get(handlers::health::health))
        .route("/_limina/health/live", get(handlers::health::live))
        .route("/_limina/health/ready", get(handlers::health::ready))
        .route("/_limina/users", get(handlers::users::users))
        .route("/_limina/version", get(handlers::version::version))
        .route("/_limina/info", get(handlers::info::info))
        .route("/_limina/metrics", get(handlers::metrics::metrics))
        .route("/_limina/routes", get(handlers::routes::routes))
        .route("/_limina/config/validate", post(handlers::config::validate))
        .route("/_limina/config/reload", post(handlers::config::reload))
}
