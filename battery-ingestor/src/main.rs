use axum::{routing::post, Json, Router};
use mongodb::{Client, options::ClientOptions};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
struct BatteryData {
    id: String,
    voltage: f32,
    current: f32,
    temp: f32,
    score: f32,
    status: String,
}

#[tokio::main]
async fn main() {
    // 1. Connect to MongoDB (Ensure MongoDB is running first!)
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.expect("Failed to parse options");
    let client = Client::with_options(client_options).expect("Failed to create client");
    let db = client.database("BatterySentinel");
    let collection = db.collection::<serde_json::Value>("telemetry");

    // 2. Define the API Route
    let app = Router::new().route("/telemetry", post(move |Json(payload): Json<BatteryData>| {
        let coll = collection.clone();
        async move {
            println!("📥 Received from: {} | Status: {}", payload.id, payload.status);
            
            // Enrichment: Add a global timestamp for Spark analysis
            let mut doc = serde_json::to_value(&payload).unwrap();
            doc.as_object_mut().unwrap().insert("timestamp".to_string(), serde_json::json!(Utc::now().to_rfc3339()));

            // Save to Distributed DB
            coll.insert_one(doc).await.expect("Failed to insert into MongoDB");
            "ACK: Data Stored"
        }
    }));

    // 3. Start the Server on port 8080
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Rust Ingestor active at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
