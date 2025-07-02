use axum::Router;

use crate::{middlewares, routes};

pub async fn create_app() -> Router {
    routes::configure_routes()
        .layer(middlewares::get_cors())
}
