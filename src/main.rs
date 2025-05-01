mod db;
mod lib;
mod module;
mod services;
mod struct_todo;

use crate::module::handle_list::{add_todo, get_list};
use axum::{
    Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/list", get(get_list))
        .route("/add", post(add_todo));

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
