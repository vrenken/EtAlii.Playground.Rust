mod data;
use data::*;

mod portal;
use portal::dashboard::*;
use portal::items::*;
use portal::input::*;

use askama::Template;
use axum::{
    routing::{get, post},
    extract::{Form, State},
    response::Html,
    Router,
};
use serde::Deserialize;
use std::sync::{Arc, Mutex};


async fn update_name(
    State(state): State<AppState>,
    Form(form): Form<NameForm>,
) -> Html<String> {
    *state.name.lock().unwrap() = form.value.clone();
    Html(form.value)
}

#[derive(Deserialize)]
struct ItemForm {
    name: String,
    quantity: u32
}

async fn item_list(State(state): State<AppState>) -> Html<String> {
    let items = state.items.lock().unwrap();
    Html(
        ItemListTemplate {
            title: "Welcome",
            subtitle: "to our list",
            items: &items }
            .render().unwrap()
    )
}

async fn add_item(
    State(state): State<data::AppState>,
    Form(form): Form<ItemForm>) -> Html<String>
{

    state.items.lock().unwrap().push(ListItem::new(&form.name, form.quantity));
    //item_list(State(state)).await
    let items = state.items.lock().unwrap();

    let items_html: String = items
        .iter()
        .map(|item| ItemTemplate { item }.render().unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    Html(items_html)
}

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
        .route("/", get(home))
        .route("/input", get(show))
        .route("/update-name", post(update_name))
        .route("/items/add", post(add_item))
        .route("/items", get(item_list))
        .with_state(state)
        .nest_service("/static", axum::routing::get_service(
            tower_http::services::ServeDir::new("static")
        ));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
