use axum::{http::StatusCode, response::Json};
use serde_json::json;


pub async fn hello_world_handler()  -> (StatusCode, Json<serde_json::Value>) {
    let data = json!({
        "message": "Hello, world!",
        "status": "success"
    });

    (StatusCode::OK, Json(data))
}


pub async fn todo_handler() -> (StatusCode, Json<serde_json::Value>) {
    let data = json!({
        "message": "Todo!.",
        "status": "success"
    });

    (StatusCode::OK, Json(data))
}
