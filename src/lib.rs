pub mod app;
pub mod http;
pub mod service;
pub mod state;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
