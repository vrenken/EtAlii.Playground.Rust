use axum::{
    extract::Form,
    response::{Html, Redirect},
    routing::get,
    Router,
};

use axum::routing::MethodRouter;
use axum_sessions::{
    async_session::MemoryStore,
    SessionLayer, SessionHandle,
};
use crate::data::AppState;
use crate::portal::dashboard::{get_cpu, get_page, get_ram};

pub fn router() -> Router<AppState> {
//     let store = MemoryStore::new();
//     let session_layer = SessionLayer::new(store, b"secretkeythatissafe");
//
     Router::new()
//         //.route("/", get(index))
//         .route("/login", get(show_login).post(process_login))
//         .layer(session_layer)
}

// Example handlers
// async fn index(session: SessionHandle) -> Html<String> {
//     if let Some(username) = session.get::<String>("user")
//     {
//         Html(format!("Hello, {}!", username))
//     } else
//     {
//         Html("Hello, guest!".to_string())
//     }
// }

async fn show_login() -> Html<&'static str>
{
    Html(r#"<form method="post" action="/login">
        <input name="username">
        <input type="password" name="password">
        <button type="submit">Login</button>
    </form>"#)
}

#[derive(serde::Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

// async fn process_login(
//     Form(form): Form<LoginForm>,
//     session: SessionHandle,
// ) -> Redirect {
//     // TODO: validate username/password
//     if form.username == "user" && form.password == "pass" {
//         session.insert("user", form.username).unwrap();
//         Redirect::to("/")
//     } else {
//         Redirect::to("/login")
//     }
// }
