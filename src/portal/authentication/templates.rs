use askama::Template;
use axum_messages::Message;

#[derive(Template)]
#[template(path = "authentication//login.html")]
pub struct LoginTemplate<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub messages: Vec<Message>,
    pub next: Option<String>,
    pub is_authenticated: bool,
}


// #[derive(Template)]
// #[template(path = "authentication//protected.html")]
// pub struct ProtectedTemplate<'a> {
//     pub title: &'a str,
//     pub subtitle: &'a str,
//     pub messages: Vec<Message>,
//     pub username: &'a str,
//     pub is_authenticated: bool,
// }
