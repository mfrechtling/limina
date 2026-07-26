use axum::Router;
use tokio::net::TcpListener;

use crate::service;
use crate::state::AppState;

pub struct App {
    router: Router,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::from_state(AppState::new()))
    }

    pub fn from_state(state: AppState) -> Self {
        let router = service::http(state);

        Self { router }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("0.0.0.0:8080").await?;

        println!("Limina listening on 0.0.0.0:8080");

        axum::serve(listener, self.router).await?;

        Ok(())
    }
}
