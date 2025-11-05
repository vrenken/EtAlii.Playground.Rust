use crate::data::AppState;
use crate::portal::*;

use askama::Template;
use axum::extract::State;
use axum::response::Html;
use axum::Form;
use axum::Router;
use axum::routing::get;
use axum::routing::post;

pub fn route_private(app_router: Router<AppState>) -> Router<AppState> {
    app_router
        .route("/input", get(get_page))
        .route("/input/update-name", post(update_name))
}

pub async fn get_page(State(state): State<AppState>) -> Html<String> {
    let name = state.name.lock().unwrap().clone();
    Html(
        input::PageTemplate {
            title: "Welcome",
            subtitle: "to our page",
            value: &name,
            is_authenticated: state.is_authenticated.lock().unwrap().clone(),
        }.render().unwrap()
    )
}

pub async fn update_name(
    State(state): State<AppState>,
    Form(form): Form<NameForm>,
) -> Html<String> {
    *state.name.lock().unwrap() = form.value.clone();
    Html(form.value)
}