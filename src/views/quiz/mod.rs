use axum::{Router, routing::get};

use crate::state::AppState;

mod create;
mod init;
mod list;

pub fn configure_views() -> Router<AppState> {
    Router::new()
        .route("/list", get(list::list))
        .route("/create", get(create::create))
        .route("/init", get(init::init))
}
