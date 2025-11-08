use axum::Router;
use std::sync::{Arc, Mutex};
use tokio::signal;
use tokio::task::AbortHandle;
use tracing_subscriber::EnvFilter;

mod routing_error;

mod data;
use data::*;

mod service;
mod configuration;
mod portal;
use portal::*;

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

    // tracing_subscriber::registry()
    //     .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
    //         |_| "axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug".into(),
    //     )))
    //     .with(tracing_subscriber::fmt::layer())
    //     .try_init()?;


    tracing::info!("Starting");

    // === Configuration.
    let configuration = configuration::setup();
    tracing::info!("Loading configuration for {}", configuration.application.name);

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
        is_authenticated: Arc::new(Mutex::new(false))
    };

    tracing::info!("Setting up tokio router");
    let mut app_router = Router::new();

    // Register protected routes.
    app_router = items::route_private(app_router).await.unwrap();
    app_router = input::route_private(app_router).await.unwrap();
    
    // Register authentication.
    let auth = authentication::route_authentication(app_router).await.unwrap();
    
    // Register public routes.
    let secured_app_router = dashboard::route_public(auth.secured_app_router).await.unwrap();
    
    let app = secured_app_router
        .with_state(state)
        .with_state(configuration)
        .nest_service("/static", axum::routing::get_service(tower_http::services::ServeDir::new("static")))
        .nest_service("/favicon.ico", axum::routing::get_service(tower_http::services::ServeFile::new("favicon.ico")));

    // run our app with hyper, listening globally on port 3000
    let endpoint = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(endpoint).await?;
    tracing::info!("listening on http://{}", endpoint.replace("0.0.0.0", "127.0.0.1"));
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(auth.deletion_task_abort_handle))
        .await?;

    Ok(())
}


async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}