use reqwest::StatusCode;
use serde_json::json;

// 1. Removed SocketAddr (unused)
// 2. If you aren't using spawn_app yet, comment it out or call it!

#[tokio::test]
async fn test_telemetry_ingestion_success() {
    let client = reqwest::Client::new();
    
    // If your server is running in another terminal, this works.
    let server_url = "http://127.0.0.1:8080/telemetry"; 

    let test_data = json!({
        "id": "ESP32_TEST_NODE",
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
        .expect("Failed to execute request. Is your server running?");

    assert_eq!(response.status(), StatusCode::OK);
}