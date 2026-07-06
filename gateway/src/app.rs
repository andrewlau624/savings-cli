use axum::Router;
use axum::routing::{get, post};

use crate::api;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(api::health::check))
        .route("/v1/chat/completions", post(api::chat::completions))
        .with_state(state)
}
