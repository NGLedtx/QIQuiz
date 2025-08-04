use std::sync::Arc;

use axum::Router;

use crate::{connections, middlewares, routes, state::AppState};

pub async fn create_app() -> Router {
    connections::init_connections().await;

    let db_conn = Arc::new(connections::get_database_connection());

    let state = AppState { db_conn };

    routes::configure_routes()
        .layer(middlewares::get_cors())
        .with_state(state)
}
