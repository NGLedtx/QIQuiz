use axum::Router;

use crate::{state::AppState, views};

mod user;

pub fn configure_routes() -> Router<AppState> {
    Router::new().merge(views::configure_views()).nest(
        "/api",
        Router::new().nest("/user", user::configure_routes()),
    )
}
