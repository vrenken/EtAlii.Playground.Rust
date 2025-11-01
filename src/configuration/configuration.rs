use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    application: Application,
}

#[derive(Debug, Deserialize)]
pub struct Application {
    name: String,
    log_level: String,
}