mod handler;
mod routes;

#[tokio::main]
async fn main() {
    let app = routes::create_api_route(); // Create the service, with the handler

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(); // Create a listener

    axum::serve(listener, app).await.unwrap(); // Serve the service through the listener
}
