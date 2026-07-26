use std::sync::Arc;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct AppState {
    pub health_response: String,
    pub version: String,
    pub service_name: String,
    pub environment: String,
    pub instance_id: String,
    pub started_at: Instant,
    pub management_routes: Vec<String>,
    pub current_config: Arc<RwLock<GatewayConfig>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GatewayConfig {
    pub mode: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            health_response: "ok".to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            service_name: "limina".to_owned(),
            environment: "development".to_owned(),
            instance_id: "limina-local".to_owned(),
            started_at: Instant::now(),
            management_routes: vec![
                "/_limina/health".to_owned(),
                "/_limina/health/live".to_owned(),
                "/_limina/health/ready".to_owned(),
                "/_limina/version".to_owned(),
                "/_limina/info".to_owned(),
                "/_limina/metrics".to_owned(),
                "/_limina/routes".to_owned(),
                "/_limina/config/validate".to_owned(),
                "/_limina/config/reload".to_owned(),
                "/_limina/users".to_owned(),
            ],
            current_config: Arc::new(RwLock::new(GatewayConfig {
                mode: "normal".to_owned(),
            })),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
