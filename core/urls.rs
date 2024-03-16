use axum::{routing::get, Router};
use crate::core::handlers;

pub fn urls () -> Router {
    let app = Router::new()
        .route("/", get(handlers::hello_world_handler))
        .route("/todo", get(handlers::todo_handler));
    return app
}