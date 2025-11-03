use config::{File, Environment};
use axum::{ routing::{get, post}, Router };
use std::sync::{Arc, Mutex};
use tracing::*;
use tracing_subscriber::EnvFilter;

mod data;
use data::*;

mod service;

mod configuration;
mod portal;

use portal::dashboard::*;
use portal::items::*;
use portal::input::*;


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Initialize tracing for console output
    tracing_subscriber::fmt()
        .compact()
        //.pretty()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("trace".parse()?)) // fallback filter
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .init();

    info!("Starting application...");

    // === Configuration.
    let configuration = configuration::setup();

    tracing::info!("Creating app state");

    let items = vec![
        ListItem::new("Milk", 2),
        ListItem::new("Bread", 1),
        ListItem::new("Eggs", 12),
        ListItem::new("Butter", 1),
        ListItem::new("Coffee", 3),
    ];
    let state = AppState {
        items: Arc::new(Mutex::new(items)),
        name: Arc::new(Mutex::new(String::from(""))),
    };

    tracing::info!("Setting up tokio router");
    let app = Router::new()
        .route("/", get(portal::dashboard::get_page))
        .route("/dashboard/cpu", get(get_cpu))
        .route("/dashboard/ram", get(get_ram))
        .route("/input", get(portal::input::get_page))
        .route("/input/update-name", post(update_name))
        .route("/items/add", post(add_item))
        .route("/items", get(portal::items::get_page))
        .with_state(state)
        .with_state(configuration)
        .nest_service("/static", axum::routing::get_service(tower_http::services::ServeDir::new("static")))
        .nest_service("/favicon.ico", axum::routing::get_service(tower_http::services::ServeFile::new("favicon.ico")));

    // run our app with hyper, listening globally on port 3000
    let endpoint = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(endpoint).await?;
    tracing::info!("listening on http://{}", endpoint.replace("0.0.0.0", "127.0.0.1"));
    axum::serve(listener, app).await?;

    Ok(())
}
