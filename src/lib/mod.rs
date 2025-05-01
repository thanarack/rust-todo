use axum::response::Json;
use serde_json::{Value, json};

pub async fn reponse_json<T: serde::Serialize>(data: Option<T>, message: Option<&'static str>) -> Json<Value> {
    Json(json!({
        "data": data,
        "message": message.unwrap_or_default()
    }))
}
