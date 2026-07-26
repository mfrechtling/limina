use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use reqwest::StatusCode;
use reqwest::blocking::{Client, Response};
use reqwest::header::CONTENT_TYPE;

fn start_server() -> Child {
    Command::new(env!("CARGO_BIN_EXE_limina"))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start limina")
}

fn wait_for_health(client: &Client) {
    for _ in 0..60 {
        if let Ok(response) = client.get("http://localhost:8080/_limina/health").send() {
            if response.status() == StatusCode::OK {
                return;
            }
        }
        thread::sleep(Duration::from_millis(250));
    }

    panic!("Timed out waiting for Limina health endpoint");
}

fn content_type(response: &Response) -> String {
    response
        .headers()
        .get(CONTENT_TYPE)
        .expect("Missing content-type header")
        .to_str()
        .expect("Invalid content-type header")
        .to_owned()
}

fn is_semver_like(version: &str) -> bool {
    let core = version.split(['-', '+']).next().unwrap_or("");
    let parts: Vec<&str> = core.split('.').collect();

    if parts.len() != 3 {
        return false;
    }

    parts
        .iter()
        .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
}

#[test]
fn management_endpoints_contract() {
    let mut server = start_server();
    let client = Client::new();

    wait_for_health(&client);

    let health = client
        .get("http://localhost:8080/_limina/health")
        .send()
        .expect("health request failed");
    assert_eq!(health.status(), StatusCode::OK);
    assert!(content_type(&health).starts_with("text/plain"));
    assert_eq!(health.text().expect("health body read failed"), "ok");

    let live = client
        .get("http://localhost:8080/_limina/health/live")
        .send()
        .expect("live request failed");
    assert_eq!(live.status(), StatusCode::OK);
    assert!(content_type(&live).starts_with("text/plain"));
    assert_eq!(live.text().expect("live body read failed"), "ok");

    let ready = client
        .get("http://localhost:8080/_limina/health/ready")
        .send()
        .expect("ready request failed");
    assert_eq!(ready.status(), StatusCode::OK);
    assert!(content_type(&ready).starts_with("application/json"));
    assert_eq!(
        ready.text().expect("ready body read failed"),
        "{\"ready\":true}"
    );

    let version = client
        .get("http://localhost:8080/_limina/version")
        .send()
        .expect("version request failed");
    assert_eq!(version.status(), StatusCode::OK);
    assert!(content_type(&version).starts_with("text/plain"));
    let version_body = version.text().expect("version body read failed");
    assert!(!version_body.is_empty());
    assert!(is_semver_like(&version_body));

    let metrics = client
        .get("http://localhost:8080/_limina/metrics")
        .send()
        .expect("metrics request failed");
    assert_eq!(metrics.status(), StatusCode::OK);
    assert!(content_type(&metrics).starts_with("text/plain"));
    let metrics_body = metrics.text().expect("metrics body read failed");
    assert!(metrics_body.contains("# HELP limina_up Gateway process health"));
    assert!(metrics_body.contains("limina_up 1"));
    assert!(metrics_body.contains("limina_uptime_seconds"));

    let routes = client
        .get("http://localhost:8080/_limina/routes")
        .send()
        .expect("routes request failed");
    assert_eq!(routes.status(), StatusCode::OK);
    assert!(content_type(&routes).starts_with("application/json"));
    let routes_body = routes.text().expect("routes body read failed");
    assert!(routes_body.contains("/_limina/health"));
    assert!(routes_body.contains("/_limina/health/live"));
    assert!(routes_body.contains("/_limina/health/ready"));
    assert!(routes_body.contains("/_limina/version"));
    assert!(routes_body.contains("/_limina/metrics"));

    let unknown = client
        .get("http://localhost:8080/unknown")
        .send()
        .expect("unknown request failed");
    assert_eq!(unknown.status(), StatusCode::NOT_FOUND);

    let validate_wrong_method = client
        .get("http://localhost:8080/_limina/config/validate")
        .send()
        .expect("validate wrong method request failed");
    assert_eq!(
        validate_wrong_method.status(),
        StatusCode::METHOD_NOT_ALLOWED
    );

    let reload_wrong_method = client
        .get("http://localhost:8080/_limina/config/reload")
        .send()
        .expect("reload wrong method request failed");
    assert_eq!(reload_wrong_method.status(), StatusCode::METHOD_NOT_ALLOWED);

    server.kill().ok();
    server.wait().ok();
}
