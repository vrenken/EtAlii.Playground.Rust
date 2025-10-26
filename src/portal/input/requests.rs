use crate::data::*;
use crate::portal::*;

use askama::Template;
use axum::{extract::State, response::Html, Form};

pub async fn input(State(state): State<AppState>) -> Html<String> {
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

pub async fn update_name(
    State(state): State<AppState>,
    Form(form): Form<NameForm>,
) -> Html<String> {
    *state.name.lock().unwrap() = form.value.clone();
    Html(form.value)
}
