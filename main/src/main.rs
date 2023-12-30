use auth::Backend;
use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, response::IntoResponse, routing::get,
    BoxError, Router,
};
use axum_login::{tower_sessions::SessionManagerLayer, AuthManagerLayerBuilder};

use environment::load_environment;
use github_webhook_handler::handler::{github_webhook_handler, GithubWebhookHandler};
use mongo_push_repository::mongo_push_repository::MongoPushRepository;
use octocrab::Octocrab;
use review_stream_service::service::ReviewStreamService;
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;

use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore};
use web_htmx::{livereload, routes as web_routes, state::WebHtmxState};

mod auth;
mod environment;

#[tokio::main]
async fn main() {
    // Bootstrapping out logging/tracing infra
    // https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/fn.init.html
    tracing_subscriber::fmt::init();

    let env = load_environment();

    // Github client
    let app_id = env
        .github_app_config
        .app_id
        .parse::<u64>()
        .expect("Failed to parse Github App ID")
        .into();
    let app_private_key_path = env.github_app_config.private_key_path;
    let app_private_key =
        std::fs::read_to_string(app_private_key_path).expect("Failed to read private key pem file");
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(app_private_key.as_bytes())
        .expect("Failed to generate JWT from pem");
    let octocrab = Octocrab::builder()
        .app(app_id, key)
        .build()
        .expect("Could not init github client");

    // Create review stream service
    let push_repository = Arc::new(
        MongoPushRepository::new(&env.review_stream_config.mongo_url)
            .await
            .expect("Could not create push repository"),
    );

    let review_stream_service = ReviewStreamService::new(push_repository, octocrab.clone());
    let github_webhook_handler_state = GithubWebhookHandler {
        review_stream_service: Arc::new(review_stream_service),
        octocrab_client: octocrab,
    };

    // Create WebHtmxState
    // This is how you can inject dependencies into the web-htmx crate
    // like a backend service
    // TODO: include an example
    let web_htmx_state = WebHtmxState {
        flash_config: axum_flash::Config::new(axum_flash::Key::generate()),
    };

    let app = Router::new()
        .merge(web_routes(web_htmx_state))
        .merge(github_webhook_handler(github_webhook_handler_state))
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
