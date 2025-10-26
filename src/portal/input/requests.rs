use crate::data::*;
use crate::portal::*;

use askama::Template;
use axum::{
    extract::State,
    response::Html,
};

pub async fn show(State(state): State<AppState>) -> Html<String> {
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
