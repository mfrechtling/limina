use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::Serialize;

use crate::state::{AppState, GatewayConfig};

#[derive(Debug, Serialize)]
pub struct ConfigValidationResponse {
    pub valid: bool,
    pub errors: Vec<String>,
}

pub async fn validate(
    State(_state): State<AppState>,
    Json(candidate): Json<GatewayConfig>,
) -> Json<ConfigValidationResponse> {
    Json(validate_config(&candidate))
}

pub async fn reload(
    State(state): State<AppState>,
    Json(candidate): Json<GatewayConfig>,
) -> (StatusCode, Json<ConfigValidationResponse>) {
    let validation = validate_config(&candidate);

    if !validation.valid {
        return (StatusCode::BAD_REQUEST, Json(validation));
    }

    let mut config = state.current_config.write().await;
    *config = candidate;

    (StatusCode::OK, Json(validation))
}

fn validate_config(candidate: &GatewayConfig) -> ConfigValidationResponse {
    let mode = candidate.mode.as_str();
    let supported = ["normal", "read-only", "fail-open", "fail-closed"];

    if supported.contains(&mode) {
        ConfigValidationResponse {
            valid: true,
            errors: Vec::new(),
        }
    } else {
        ConfigValidationResponse {
            valid: false,
            errors: vec![format!("unsupported mode: {mode}")],
        }
    }
}