use axum::Router;

use crate::state::AppState;

pub fn configure_routes() -> Router<AppState> {
    Router::new()
}
