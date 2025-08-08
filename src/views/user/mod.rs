use axum::{Router, routing::get};

use crate::state::AppState;

mod login;

pub fn configure_views() -> Router<AppState> {
    Router::new().route("/login", get(login::login))
}
