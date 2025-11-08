use crate::data::AppState;
use crate::portal::*;
use crate::routing_error::RouteError;

use askama::Template;
use axum::extract::State;
use axum::response::Html;
use axum::Form;
use axum::Router;
use axum::routing::get;
use axum::routing::post;

pub async fn route_private(app_router: Router<AppState>) -> Result<Router<AppState>, RouteError> {
    let result = app_router
        .route("/items/add", post(add_item))
        .route("/items", get(get_page));
    Ok(result)
}

pub async fn get_page(State(state): State<AppState>) -> Html<String> {
    let items = state.items.lock().unwrap();
    Html(
        items::PageTemplate {
            title: "Welcome",
            subtitle: "to our list",
            items: &items,
            is_authenticated: state.is_authenticated.lock().unwrap().clone(),
        }.render().unwrap()
    )
}

pub async fn add_item(
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