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
    items: Arc<Mutex<Vec<ListItem>>>,
    name: Arc<Mutex<String>>,
}

#[derive(Template)]
#[template(path = "input.html")]
struct InputTemplate<'a> {
    title: &'a str,
    subtitle: &'a str,
    value: &'a str,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    title: &'a str,
    subtitle: &'a str,
    value: &'a str,
}

#[derive(Template)]
#[template(path = "item.html")]
struct ItemTemplate<'a> {
    item: &'a ListItem,
}

#[derive(Template)]
#[template(path = "item_list.html")]
struct ItemListTemplate<'a> {
    title: &'a str,
    subtitle: &'a str,
    items: &'a [ListItem],
}

#[derive(Deserialize)]
struct NameForm {
    value: String,
}

async fn show(State(state): State<AppState>) -> Html<String> {
    let name = state.name.lock().unwrap().clone();
    Html(
        InputTemplate {
            title: "Welcome",
            subtitle: "to our page",
            value: &name,
        }
            .render().unwrap()
    )
}

async fn home(State(state): State<AppState>) -> Html<String> {
    let name = state.name.lock().unwrap().clone();
    Html(
        HomeTemplate {
            title: "Welcome",
            subtitle: "to our page",
            value: &name,
        }
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

#[derive(Clone)]
struct ListItem {
    name: String,
    quantity: u32,
}

impl ListItem {
    fn new(name: &str, quantity: u32) -> Self {
        Self {
            name: name.to_string(),
            quantity,
        }
    }
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
    State(state): State<AppState>,
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
    axum::serve(listener, app).await.unwrap();
}
