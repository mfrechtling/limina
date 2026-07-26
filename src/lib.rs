pub mod management;
pub mod state;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
