use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    status: String,
}

pub async fn check() -> Json<Health> {
    Json(Health { status: "ok".to_string() })
}
