use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Configuration {
    pub application: Application,
    pub logging: Logging
}

#[derive(Debug, Clone, Deserialize)]
pub struct Application {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logging {
    pub log_level: String,
    pub otlp_endpoint: String,
    pub seq_api_key: String,
}

pub fn setup() -> Configuration {
    let config = Config::builder()
        .add_source(File::with_name("configuration"))
        // .add_source(File::with_name("configuration.production").required(false))
        .add_source(File::with_name("configuration.development").required(false))
        .add_source(Environment::with_prefix("APP").separator("_"))
        .build()
        .unwrap();

    tracing::info!("Loaded configuration");

    config.try_deserialize().unwrap()
}
