use askama::Template;
use axum::{
    routing::{get, post},
    extract::{Form, State},
    response::Html,
    Router,
};
use serde::Deserialize;
use std::sync::{Arc, Mutex};


#[derive(Clone)]
struct AppState {
    items: Arc<Mutex<Vec<String>>>,
    name: Arc<Mutex<String>>,
}

#[derive(Template)]
#[template(path = "input.html")]
struct InputTemplate<'a> {
    title: &'a str,
    value: &'a str,
}

#[derive(Template)]
#[template(path = "item_list.html")]
struct ItemListTemplate<'a> {
    items: &'a [String],
}

#[derive(Deserialize)]
struct NameForm {
    value: String,
}

async fn show(State(state): State<AppState>) -> Html<String> {
    let name = state.name.lock().unwrap().clone();
    Html(
        InputTemplate { title: "Welcome", value: &name }
            .render().unwrap()
    )
}

async fn update_name(
    State(state): State<AppState>,
    Form(form): Form<NameForm>,
) -> Html<String> {
    *state.name.lock().unwrap() = form.value.clone();
    Html(form.value)
}

#[derive(Deserialize)]
struct ItemForm {
    newItem: String,
}

async fn item_list(State(state): State<AppState>) -> Html<String> {
    let items = state.items.lock().unwrap();
    Html(
        ItemListTemplate { items: &items }
            .render().unwrap()
    )
}

async fn add_item(
    State(state): State<AppState>,
    Form(form): Form<ItemForm>,
) -> Html<String> {
    state.items.lock().unwrap().push(form.newItem);
    item_list(State(state)).await
}

#[tokio::main]
async fn main() {
    let state = AppState {
        items: Arc::new(Mutex::new(vec![])),
        name: Arc::new(Mutex::new(String::from(""))),
    };

    let app = Router::new()
        .route("/", get(show))
        .route("/update-name", post(update_name))
        .route("/items/add", post(add_item))
        .route("/items", get(item_list))
        .with_state(state)
        .nest_service("/static", axum::routing::get_service(
            tower_http::services::ServeDir::new("static")
        ));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
