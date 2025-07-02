use axum::Router;

use crate::{connections, middlewares, routes};

pub async fn create_app() -> Router {
    connections::init_connections().await;
    
    routes::configure_routes()
        .layer(middlewares::get_cors())
}
