use axum::{http::StatusCode, response::Json};
use serde_json::json;
use crate::core::responses::Message;

pub async fn hello_world_handler()  -> (StatusCode, Json<serde_json::Value>) {
    let data = Message {
        status: "success",
        message: "Hello, world!",
    };
    let response = json!(&data);
    (StatusCode::OK, Json(response))
}


pub async fn todo_handler() -> (StatusCode, Json<serde_json::Value>) {
    let data = Message {
        status: "success",
        message: "Todo!",
    };
    let response = json!(&data);

    (StatusCode::OK, Json(response))
}
