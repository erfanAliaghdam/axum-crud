mod handlers;
mod urls;
use crate::urls::urls;


// tokio makes async programming easier and is a stack on top of the rest
#[tokio::main]
async fn main() {
    // build our application with a route
    let app = urls();

    // serve the route
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

