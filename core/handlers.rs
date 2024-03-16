use axum::{response::Html};


pub async fn hello_world_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}


pub async fn todo_handler() -> Html<&'static str> {
    Html("<h1>Todo handler</h1>")
}
