use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/example", get(example_handler))
}

async fn example_handler() -> &'static str {
    "This is an example route!"
}
