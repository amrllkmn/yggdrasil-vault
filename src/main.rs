use axum::{routing::get, Router};
mod handler;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler::hello)); // Create the service, with the handler

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(); // Create a listener

    axum::serve(listener, app).await.unwrap(); // Serve the service through the listener
}
