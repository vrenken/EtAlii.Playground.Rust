use askama::Template;
use axum_messages::Message;

#[derive(Template)]
#[template(path = "authentication//login.html")]
pub struct LoginTemplate {
    pub messages: Vec<Message>,
    pub next: Option<String>,
}


#[derive(Template)]
#[template(path = "authentication//protected.html")]
pub struct ProtectedTemplate<'a> {
    pub messages: Vec<Message>,
    pub username: &'a str,
}
