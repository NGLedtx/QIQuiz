use axum::{Router, routing::get};

use crate::state::AppState;

mod home;
mod quiz;
mod user;

pub fn configure_views() -> Router<AppState> {
    Router::new()
        .route("/", get(home::home))
        .nest("/user", user::configure_views())
        .nest("/quiz", quiz::configure_views())
}
