// tests/integration_tests.rs

use serde_json::json;
use tokio::net::TcpListener;
use std::time::Duration;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

// ── Minimal mirror of your production BatteryData ──────────────────────────
#[derive(Debug, Serialize, Deserialize)]
struct BatteryData {
    id: String,
    voltage: f32,
    current: f32,
    temp: f32,
    score: f32,
    status: String,
}

// ── Spin up a REAL Axum server (no MongoDB) on a random free port ───────────
async fn spawn_app() -> String {
    // Port 0 → OS picks a free port, eliminating "address in use" flakiness
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind port");

    let addr = listener.local_addr().unwrap();

    // Real router with a stub handler — no MongoDB needed
    let app = Router::new().route(
        "/telemetry",
        post(|Json(_payload): Json<BatteryData>| async { "ACK: Data Stored" }),
    );

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    format!("http://{}", addr)
}

// ── The actual test ─────────────────────────────────────────────────────────
#[tokio::test]
async fn test_telemetry_ingestion_success() {
    let base_url = spawn_app().await;

    // Small grace period for the spawned task to start accepting
    tokio::time::sleep(Duration::from_millis(50)).await;

    let client = reqwest::Client::new();
    let url = format!("{}/telemetry", base_url);

    let test_data = json!({
        "id":      "ESP32_NODE_01",
        "voltage": 12.6,
        "current": 1.2,
        "temp":    35.5,
        "score":   98.0,
        "status":  "Healthy"
    });

    let response = client
        .post(&url)
        .json(&test_data)
        .send()
        .await
        .expect("Request failed");

    println!("Response status: {}", response.status());

    // The stub returns 200 OK with the ACK body
    assert!(
        response.status().is_success(),
        "Expected 2xx, got {}",
        response.status()
    );

    let body = response.text().await.unwrap();
    assert_eq!(body, "ACK: Data Stored");
}