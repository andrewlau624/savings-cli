use axum::Router;
use axum::routing::get;

use crate::api;

pub fn router() -> Router {
    Router::new().route("/health", get(api::health::check))
}
