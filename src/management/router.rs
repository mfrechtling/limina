use axum::{
    Router,
    routing::{get, post},
};

use crate::management::endpoints;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/_limina/health", get(endpoints::health::health))
    .route("/_limina/health/live", get(endpoints::health::live))
    .route("/_limina/health/ready", get(endpoints::health::ready))
        .route("/_limina/users", get(endpoints::users::users))
        .route("/_limina/version", get(endpoints::version::version))
    .route("/_limina/info", get(endpoints::info::info))
    .route("/_limina/metrics", get(endpoints::metrics::metrics))
    .route("/_limina/routes", get(endpoints::routes::routes))
    .route("/_limina/config/validate", post(endpoints::config::validate))
    .route("/_limina/config/reload", post(endpoints::config::reload))
}