use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

fn start_server() -> Child {
    Command::new(env!("CARGO_BIN_EXE_limina"))
        .spawn()
        .expect("Failed to start limina")
}

#[test]
fn unknown_route_returns_404() {
    let mut server = start_server();

    // Wait for the server to start
    thread::sleep(Duration::from_millis(500));

    let response = reqwest::blocking::get("http://localhost:8080/unknown")
        .expect("Failed to connect to Limina");

    assert_eq!(response.status(), 404);

    server.kill().ok();
    server.wait().ok();
}
