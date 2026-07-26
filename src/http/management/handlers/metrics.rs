use axum::extract::State;

use crate::state::AppState;

pub async fn metrics(State(state): State<AppState>) -> String {
    format!(
        "# HELP limina_up Gateway process health\n# TYPE limina_up gauge\nlimina_up 1\n# HELP limina_uptime_seconds Process uptime in seconds\n# TYPE limina_uptime_seconds gauge\nlimina_uptime_seconds {}\n",
        state.started_at.elapsed().as_secs()
    )
}