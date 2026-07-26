use axum::Router;

use crate::http::management;
use crate::state::AppState;

pub fn http(state: AppState) -> Router {
    Router::<AppState>::new()
        .merge(management::routes::router())
        .with_state(state)
}
