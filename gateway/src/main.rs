mod api;
mod app;
mod config;
mod models;
mod state;

use std::sync::Arc;

use config::Config;
use models::ModelsConfig;
use state::AppState;

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let config = Config::from_env().expect("failed to load config (is .env set?)");
    let models = ModelsConfig::load("models.toml").expect("failed to load models.toml");
    tracing::info!(
        "gateway starting: {} model(s), default '{}', backend {}",
        models.models.len(),
        models.default,
        config.llm_base_url
    );

    let state = AppState {
        config: Arc::new(config),
        models: Arc::new(models),
        http: reqwest::Client::new(),
    };

    let app = app::router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
