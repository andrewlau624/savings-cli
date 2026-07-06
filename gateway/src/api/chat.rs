use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde_json::Value;

use crate::state::AppState;

pub async fn completions(
    State(state): State<AppState>,
    Json(mut body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let requested = body.get("model")
        .and_then(|m| m.as_str())
        .unwrap_or(state.models.default.as_str())
        .to_string();

    let served = state
        .models
        .served_name(&requested)
        .ok_or((StatusCode::BAD_REQUEST, format!("unknown model: '{requested}'!")))?
        .to_string();

    body["model"] = Value::String(served);

    let response = state
        .http
        .post(format!("{}/chat/completions", state.config.llm_base_url))
        .bearer_auth(&state.config.llm_api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("backend request failed: {e}")))?;

    let payload: Value = response
        .json()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("bad backend response: {e}")))?;

    let total_tokens = payload
      .get("usage")
      .and_then(|u| u.get("total_tokens"))
      .and_then(|t| t.as_u64());
    
    tracing::info!(model = %requested, tokens = ?total_tokens, "chat completion");

    Ok(Json(payload))
}
