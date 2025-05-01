use crate::lib::reponse_json;
use crate::services::service_list::{add_service_todo, get_service_list};
use crate::struct_todo::struct_todo::*;

use axum::response::Json;
use serde_json::{Value, json};

pub async fn get_list() -> Json<Value> {
    let service = get_service_list().await;
    let data = service;
    let response = reponse_json(data, None).await;
    return response.clone();
}

pub async fn add_todo(Json(body): Json<TodoRequest>) -> Json<Value> {
    // This code is converting a String into a static string reference using Box::leak
    let static_todo: &'static str = Box::leak(body.todo.into_boxed_str());
    let _ = add_service_todo(static_todo).await;
    let data = json!({});
    let response = reponse_json(data, Some("success")).await;
    return response.clone();
}
