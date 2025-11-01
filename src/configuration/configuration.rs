use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Configuration {
    application: Application,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Application {
    name: String,
    log_level: String,
}