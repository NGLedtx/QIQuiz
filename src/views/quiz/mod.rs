use axum::{Router, routing::get};

use crate::state::AppState;

mod create;
mod init;
mod list;
mod play;
mod rank;

pub fn configure_views() -> Router<AppState> {
    Router::new()
        .route("/list", get(list::list))
        .route("/create", get(create::create))
        .route("/init", get(init::init))
        .route("/{quiz_id}/rank", get(rank::rank))
        .route("/{quiz_id}/play", get(play::play))
}
