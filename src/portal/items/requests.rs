use crate::data::*;
use crate::portal::*;

use askama::Template;
use axum::{extract::State, response::Html, Form};

pub async fn item_list(State(state): State<AppState>) -> Html<String> {
    let items = state.items.lock().unwrap();
    Html(
        ItemListTemplate {
            title: "Welcome",
            subtitle: "to our list",
            items: &items }
            .render().unwrap()
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
