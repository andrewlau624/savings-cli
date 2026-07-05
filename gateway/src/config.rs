pub struct Config {
    pub llm_base_url: String,
    pub llm_api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Config, std::env::VarError> {
        Ok(Config {
            llm_base_url: std::env::var("LLM_BASE_URL")?,
            llm_api_key: std::env::var("LLM_API_KEY")?,
        })
    }
}
