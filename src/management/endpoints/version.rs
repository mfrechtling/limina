use axum::extract::State;

use crate::state::AppState;

pub async fn version(State(state): State<AppState>) -> String {
    state.version
}
