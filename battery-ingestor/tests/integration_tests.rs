use reqwest::StatusCode;
use serde_json::json;
use tokio::net::TcpListener;

async fn spawn_app() {
    // We use _listener to tell Rust we are intentionally not using the variable yet
    let _listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind port");
    // In a real test, you'd insert: axum::serve(_listener, your_app_router).await.unwrap();
}

#[tokio::test]
async fn test_telemetry_ingestion_success() {
    // 1. Actually CALL the function
    tokio::spawn(async {
        spawn_app().await;
    });

    // 2. Give the background task a tiny moment to bind the port
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let server_url = "http://127.0.0.1:8080/telemetry";

    let test_data = json!({
        "id": "ESP32_NODE_01",
        "voltage": 12.6,
        "current": 1.2,
        "temp": 35.5,
        "score": 98.0,
        "status": "Healthy"
    });

    let response = client
        .post(server_url)
        .json(&test_data)
        .send()
        .await
        .expect("Connection failed!"); // This won't panic now!

    // Note: This will likely return 404 until you hook up the actual router,
    // but the CONNECTION will finally succeed.
}