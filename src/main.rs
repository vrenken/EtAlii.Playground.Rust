mod data;
use data::*;

mod service;

mod portal;
use portal::dashboard::*;
use portal::items::*;
use portal::input::*;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
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

    let app = Router::new()
        .route("/", get(dashboard))
        .route("/input", get(input))
        .route("/update-name", post(update_name))
        .route("/items/add", post(add_item))
        .route("/items", get(item_list))
        .with_state(state)
        .nest_service("/static", axum::routing::get_service(tower_http::services::ServeDir::new("static")))
        .nest_service("/favicon.ico", axum::routing::get_service(tower_http::services::ServeFile::new("favicon.ico")));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
