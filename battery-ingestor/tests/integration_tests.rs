use reqwest::StatusCode;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// Helper to spawn the app in the background for testing
async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let addr = format!("http://127.0.0.1:{}", port);

    // Get the router from your main code (Note: You might need to refactor 
    // your main.rs to export the app/router for this to work perfectly)
    // For now, let's assume we are testing the endpoint logic.
    
    tokio::spawn(async move {
        // In a real scenario, you'd call your app logic here
        // axum::serve(listener, your_router).await.unwrap();
    });

    addr
}

#[tokio::test]
async fn test_telemetry_ingestion_success() {
    // 1. Setup
    let client = reqwest::Client::new();
    let server_url = "http://127.0.0.1:8080/telemetry"; // Assuming your app is running

    // 2. Mock Data
    let test_data = json!({
        "id": "ESP32_TEST_NODE",
        "voltage": 12.6,
        "current": 1.2,
        "temp": 35.5,
        "score": 98.0,
        "status": "Healthy"
    });

    // 3. Action: Send POST request
    // NOTE: Ensure your local MongoDB is running before running this!
    let response = client
        .post(server_url)
        .json(&test_data)
        .send()
        .await
        .expect("Failed to execute request");

    // 4. Assert
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.text().await.unwrap();
    assert_eq!(body, "ACK: Data Stored");
}