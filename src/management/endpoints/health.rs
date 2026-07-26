use axum::extract::State;
use axum::Json;
use serde::Serialize;

use crate::state::AppState;

pub async fn health(State(state): State<AppState>) -> String {
    state.health_response
}

pub async fn live(State(state): State<AppState>) -> String {
    state.health_response
}

#[derive(Debug, Serialize)]
pub struct ReadyResponse {
    pub ready: bool,
}

pub async fn ready(State(_state): State<AppState>) -> Json<ReadyResponse> {
    Json(ReadyResponse { ready: true })
}
