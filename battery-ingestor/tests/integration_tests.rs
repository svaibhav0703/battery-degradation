use reqwest::StatusCode;
use serde_json::json;
use tokio::net::TcpListener;

// This function starts your server on a random available port
async fn spawn_app() {
    tokio::spawn(async move {
        // In a real project, you'd call your main app's router here.
        // For now, this just simulates the server being 'alive' 
        // to pass the connection check.
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        // logic to serve your app...
    });
}

#[tokio::test]
async fn test_telemetry_ingestion_success() {
    // 1. Call the helper to start the server internally
    // spawn_app().await; 

    let client = reqwest::Client::new();
    let server_url = "http://127.0.0.1:8080/telemetry";

    let test_data = json!({
        "id": "ESP32_TEST_NODE",
        "voltage": 12.6,
        "current": 1.2,
        "temp": 35.5,
        "score": 98.0,
        "status": "Healthy"
    });

    // Action
    let response = client
        .post(server_url)
        .json(&test_data)
        .send()
        .await
        .expect("Connection failed!");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}