use serde::{Deserialize, Serialize};
use config::{Config, ConfigError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppOptions {
    pub host: String,
    pub port: u16
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub app: AppOptions,
}

pub fn get_settings() -> Result<Settings, ConfigError> {
    //let env = env::var("ENVIRONMENT").unwrap_or_else(|_| "stage".into());

    Config::builder()
        .add_source(config::File::with_name("config/base.toml"))
        //.add_source(File::with_name(&format!("config/{}/config.toml", env)).required(false))
        .build()?
        .try_deserialize()
}
