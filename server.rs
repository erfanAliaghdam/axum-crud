mod core {
    pub mod models;
    pub mod handlers;
    pub mod responses;
    pub mod repository;
    pub mod serializers;
}
use crate::core::handlers::{
    todo_list_handler,
    hello_world_handler,
    create_todo_handler,
    update_todo_handler,
    get_todo_detail_handler,
    delete_todo_handler
};
use axum::{routing::{get, post, patch, delete}, Router};
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::sync::Arc;


pub struct AppState {
    db: MySqlPool,
}

// tokio makes async programming easier and is a stack on top of the rest
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Database connected successfully!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/", get(hello_world_handler))
        .route("/todo", get(todo_list_handler))
        .route("/todo/create", post(create_todo_handler))
        .route("/todo/:id/update", patch(update_todo_handler))
        .route("/todo/:id", get(get_todo_detail_handler))
        .route("/todo/:id/delete", delete(delete_todo_handler))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    // serve 
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
