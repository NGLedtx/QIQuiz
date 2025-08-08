use axum::{Router, routing::get};

use crate::{handlers, state::AppState};

pub(super) fn configure_routes() -> Router<AppState> {
    Router::new().route("/login", get(handlers::user::login))
}
