mod food;

use axum::{Router, response::Json, routing::get};
use serde_json::{Value, json};
use food::{get_food};

async fn json_text() -> Json<Value> {
    Json(json!({"name": "bank"}))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/json", get(json_text))
        .route("/food", get(get_food));

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            return;
        }
    };

    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap()
}
