use axum::Router;

use crate::state::AppState;

mod quiz;
mod user;

pub fn configure_views() -> Router<AppState> {
    Router::new()
        .nest("/user", user::configure_views())
        .nest("/quiz", quiz::configure_views())
}
