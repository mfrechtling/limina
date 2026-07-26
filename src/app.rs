use axum::Router;
use tokio::net::TcpListener;

use crate::state::AppState;

pub struct App {
    router: Router,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let state = AppState::new();

        let router = Router::<AppState>::new()
            .merge(crate::management::router::router())
            .with_state(state);

        Ok(Self { router })
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("0.0.0.0:8080").await?;

        println!("Limina listening on 0.0.0.0:8080");

        axum::serve(listener, self.router).await?;

        Ok(())
    }
}
