mod api;
mod app;
mod config;
mod models;

use config::Config;
use models::ModelsConfig;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env().expect("failed to load config (is .env set?)");
    let models = ModelsConfig::load("models.toml").expect("failed to load models.toml");
    println!(
        "gateway starting: {} model(s), default '{}', backend {}",
        models.models.len(),
        models.default,
        config.llm_base_url
    );

    let app = app::router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
