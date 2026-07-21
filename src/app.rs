use axum::{Router, routing::get};
use tokio::net::TcpListener;

pub struct App {
    router: Router,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let router = Router::new().route("/", get(Self::hello));

        Ok(Self { router })
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("0.0.0.0:8080").await?;

        println!("Limina listening on 0.0.0.0:8080");

        axum::serve(listener, self.router).await?;

        Ok(())
    }

    async fn hello() -> &'static str {
        "Hello limina\n"
    }
}
