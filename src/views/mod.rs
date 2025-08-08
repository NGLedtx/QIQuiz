use axum::Router;

use crate::state::AppState;

pub mod user;

pub fn configure_views() -> Router<AppState> {
    Router::new().nest("/user", user::configure_views())
}
