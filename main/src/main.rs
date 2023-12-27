use auth::Backend;
use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, response::IntoResponse, routing::get,
    BoxError, Router,
};
use axum_login::{tower_sessions::SessionManagerLayer, AuthManagerLayerBuilder};

use environment::load_environment;
use std::net::SocketAddr;
use tower::ServiceBuilder;

use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore};
use web_htmx::{livereload, routes as web_routes, state::WebHtmxState};

mod auth;
mod environment;
mod github_webhook_adapter;

#[tokio::main]
async fn main() {
    // Bootstrapping out logging/tracing infra
    // https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/fn.init.html
    tracing_subscriber::fmt::init();

    let _env = load_environment();

    // Create WebHtmxState
    // This is how you can inject dependencies into the web-htmx crate
    // like a backend service
    // TODO: include an example
    let web_htmx_state = WebHtmxState {
        flash_config: axum_flash::Config::new(axum_flash::Key::generate()),
    };

    let app = Router::new()
        .merge(web_routes(web_htmx_state))
        .merge(github_webhook_adapter::github_webhook_handler())
        .route("/healthcheck", get(get_health_check));

    // Auth and session setup
    let session_store = MemoryStore::default();
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(SessionManagerLayer::new(session_store.clone()).with_secure(false));

    let user_memory_store = Backend::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));
    let auth_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(AuthManagerLayerBuilder::new(user_memory_store, session_layer).build());
    let app = app.layer(auth_layer);
    let app = app.layer(session_service);

    #[cfg(debug_assertions)]
    let app = app.layer(livereload::layer());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn get_health_check() -> impl IntoResponse {
    "OK"
}
