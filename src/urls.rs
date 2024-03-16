use axum::{routing::get, Router};
use crate::handlers::{hello_world_handler, todo_handler};


pub fn urls () -> Router {
    let app = Router::new()
        .route("/", get(hello_world_handler))
        .route("/todo", get(todo_handler));
    return app
}