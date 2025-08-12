use axum::{
    Router,
    routing::{get, post},
};

use crate::{handlers, state::AppState};

pub(super) fn configure_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handlers::quiz::create))
        .route("/{quiz_id}/delete", get(handlers::quiz::delete))
        .route("/submit", post(handlers::quiz::submit))
}
