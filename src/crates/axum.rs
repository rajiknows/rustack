//! axum crates boilerplate

pub fn write_axum() -> &'static str {
    r#"use axum::{routing::get, Router};
use tokio::net::TcpListener;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
let app = Router::new()
.route("/", get(|| async { "Hello, Rustack!" }))
.nest("/api", routes::example::router());

let listener = TcpListener::bind("0.0.0.0:3000").await?;
axum::serve(listener, app).await?;

Ok(())
}
"#
}

pub fn write_axum_routes() -> &'static str {
    r#"use axum::{routing::get, Router};

pub fn router() -> Router {
Router::new().route("/example", get(example_handler))
}

async fn example_handler() -> &'static str {
"This is an example route!"
}
"#
}
