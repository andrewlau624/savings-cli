use std::sync::Arc;

use crate::config::Config;
use crate::models::ModelsConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub models: Arc<ModelsConfig>,
    pub http: reqwest::Client,
}
