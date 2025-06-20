use axum::{routing::get, Router};
use tokio::net::TcpListener;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(|| async { concat!("Hello, ", "World ", "!") }))
        .nest("/api", routes::example::router());

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
