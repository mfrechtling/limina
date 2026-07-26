use axum::Json;
use axum::extract::State;
use serde::Serialize;

use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct InfoResponse {
    pub name: String,
    pub environment: String,
    pub instance_id: String,
    pub uptime_seconds: i64,
}

pub async fn info(State(state): State<AppState>) -> Json<InfoResponse> {
    Json(InfoResponse {
        name: state.service_name,
        environment: state.environment,
        instance_id: state.instance_id,
        uptime_seconds: state.started_at.elapsed().as_secs() as i64,
    })
}
