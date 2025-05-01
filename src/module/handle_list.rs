use crate::lib::reponse_json;
use crate::services::service_list::{add_service_todo, get_service_list};
use crate::struct_type::struct_todo::*;

use axum::response::Json;
use serde_json::Value;

pub async fn get_list() -> Json<Value> {
    let data = get_service_list().await;
    reponse_json(Some(data), None).await
}

pub async fn add_todo(Json(body): Json<TodoRequest>) -> Json<Value> {
    let todo = Box::leak(body.todo.into_boxed_str());
    let _ = add_service_todo(todo).await;
    reponse_json(Some(()), Some("success")).await
}
