use std::sync::Arc;

use axum::Router;
use reqwest::Client;

use crate::{connections, middlewares, routes, state::AppState};

pub async fn create_app() -> Router {
    connections::init_connections().await;

    let db_conn = Arc::new(connections::get_database_connection());
    let http_client = Arc::new(Client::new());

    let state = AppState {
        db_conn,
        http_client,
    };

    routes::configure_routes()
        .layer(middlewares::get_cors())
        .with_state(state)
}
