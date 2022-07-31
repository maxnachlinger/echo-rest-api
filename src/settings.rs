use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppOptions {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub app: AppOptions,
}

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8080;

pub fn get_settings() -> Settings {
    let host = option_env!("HOST").unwrap_or(DEFAULT_HOST).into();
    let port = option_env!("PORT")
        .unwrap_or(DEFAULT_PORT.to_string().as_ref())
        .parse::<u16>()
        .unwrap_or(DEFAULT_PORT);

    Settings {
        app: AppOptions { port, host },
    }
}
