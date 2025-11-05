use askama::Template;
use axum::Form;
use axum::Router;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::get;
use axum::routing::post;
use axum_login::AuthManagerLayerBuilder;
use axum_login::login_required;
use axum_login::tower_sessions::ExpiredDeletion;
use axum_login::tower_sessions::Expiry;
use axum_login::tower_sessions::SessionManagerLayer;
use axum_messages::Messages;
use axum_messages::MessagesManagerLayer;
use crate::data::AppState;
use crate::portal::authentication::templates::{LoginTemplate, ProtectedTemplate};
use crate::users::Backend;
use serde::Deserialize;
use sqlx::SqlitePool;
use time::Duration;
use tokio::task;
use tower_sessions::cookie::Key;
use tower_sessions_sqlx_store::SqliteStore;

// This allows us to extract the "next" field from the query string. We use this
// to redirect after login.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub async fn route_authentication(app_router: Router<AppState>) -> Router<AppState> {
    let db = SqlitePool::connect(":memory:").await.unwrap();
    sqlx::migrate!().run(&db).await.unwrap();

    // Session layer.
    //
    // This uses `tower-sessions` to establish a layer that will provide the session
    // as a request extension.
    let session_store = SqliteStore::new(db.clone());
    session_store.migrate().await.unwrap();

    let _deletion_task = task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    // Generate a cryptographic key to sign the session cookie.
    let key = Key::generate();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)))
        .with_signed(key);

    // Auth service.
    //
    // This combines the session layer with our backend to establish the auth
    // service which will provide the auth session as a request extension.
    let backend = Backend::new(db);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    app_router
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/login", post(post::login))
        .route("/login", get(get::login))
        .route("/logout", get(get::logout))
        .layer(MessagesManagerLayer)
        .layer(auth_layer)
}

mod post {
    use super::*;
    use crate::portal::authentication::users::{AuthSession, Credentials};

    pub async fn login(mut auth_session: AuthSession, messages: Messages, Form(creds): Form<Credentials>) -> impl IntoResponse {
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                messages.error("Invalid credentials");

                let mut login_url = "/login".to_string();
                if let Some(next) = creds.next {
                    login_url = format!("{login_url}?next={next}");
                };

                return Redirect::to(&login_url).into_response();
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if auth_session.login(&user).await.is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        messages.success(format!("Successfully logged in as {}", user.username));

        if let Some(ref next) = creds.next
        {
            Redirect::to(next)
        }
        else
        {
            Redirect::to("/")
        }.into_response()
    }
}

mod get {
    use super::*;
    use crate::portal::authentication::users::AuthSession;

    pub async fn protected(auth_session: AuthSession, messages: Messages) -> impl IntoResponse
    {
        match auth_session.user
        {
            Some(user) => Html(ProtectedTemplate {
                    title: "Protected",
                    subtitle: "You are logged in",
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

    pub async fn login(messages: Messages, Query(NextUrl { next }): Query<NextUrl>) -> Html<String>
    {
        Html(LoginTemplate {
            title: "Login",
            subtitle: "Please login",
            messages: messages.into_iter().collect(), next
        }.render().unwrap())
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}