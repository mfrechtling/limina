use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

fn start_server() -> Child {
    Command::new(env!("CARGO_BIN_EXE_limina"))
        .spawn()
        .expect("Failed to start limina")
}

#[test]
fn application_starts_and_serves_http() {
    let mut server = start_server();

    thread::sleep(Duration::from_millis(500));

    let response =
        reqwest::blocking::get("http://localhost:8080/").expect("Failed to connect to Limina");

    assert_eq!(response.status(), 200);

    let body = response.text().unwrap();

    assert_eq!(body, "Hello limina");

    server.kill().ok();
}
