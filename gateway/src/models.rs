use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ModelsConfig {
    pub default: String,
    pub models: Vec<Model>,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub name: String,
    pub served_name: String,
}

impl ModelsConfig {
    pub fn load(path: &str) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading model registry at {path}"))?;
        let config: ModelsConfig =
            toml::from_str(&text).with_context(|| format!("parsing {path}"))?;
        Ok(config)
    }

    pub fn served_name(&self, name: &str) -> Option<&str> {
        self.models
            .iter()
            .find(|m| m.name == name)
            .map(|m| m.served_name.as_str())
    }
}
