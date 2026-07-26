use axum::Json;
use axum::extract::State;
use serde::Serialize;

use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct RoutesResponse {
    pub routes: Vec<String>,
}

pub async fn routes(State(state): State<AppState>) -> Json<RoutesResponse> {
    Json(RoutesResponse {
        routes: state.management_routes,
    })
}
