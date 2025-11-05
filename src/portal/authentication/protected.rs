use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_messages::{Message, Messages};

use portal::authentication::templates::ProtectedTemplate;
use crate::portal;

pub fn router() -> Router<()> {
    Router::new().route("/", get(self::get::protected))
}

mod get {
    pub use crate::portal::authentication::users::AuthSession;
    use super::*;

    pub async fn protected(auth_session: AuthSession, messages: Messages) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => Html(
                ProtectedTemplate {
                    messages: messages.into_iter().collect(),
                    username: &user.username,
                }
                    .render()
                    .unwrap(),
            )
                .into_response(),

            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}