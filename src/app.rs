use axum::{Router, routing::get};

pub async fn create_app() -> Router {
    Router::new().route("/", get(async || "Hello"))
}
