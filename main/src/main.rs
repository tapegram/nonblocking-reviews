use auth_service::service::AuthService;
use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, response::IntoResponse, routing::get,
    BoxError, Router,
};
use axum_login::{tower_sessions::SessionManagerLayer, AuthManagerLayerBuilder};

use environment::load_environment;
use github_webhook_handler::handler::{github_webhook_handler, GithubWebhookHandler};
use mongo_feed_user_repository::mongo_user_repository::MongoFeedUserRepository;
use mongo_push_repository::mongo_push_repository::MongoPushRepository;
use mongo_user_repository::{MongoUserRepository, MongoUserStore};
use review_stream_service::service::ReviewStreamService;
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;

use tower_sessions::{
    cookie::{time::Duration, SameSite},
    mongodb::Client,
    Expiry, MongoDBStore,
};
use web_htmx::{livereload, routes as web_routes, state::WebHtmxState};

mod environment;

#[tokio::main]
async fn main() {
    // Bootstrapping out logging/tracing infra
    // https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/fn.init.html
    tracing_subscriber::fmt::init();

    let env = load_environment();

    // Create review stream service
    let push_repository = Arc::new(
        MongoPushRepository::new(&env.review_stream_config.mongo_url)
            .await
            .expect("Could not create push repository"),
    );

    let feed_user_repository = Arc::new(
        MongoFeedUserRepository::new(&env.review_stream_config.mongo_url)
            .await
            .expect("Could not create feed user repository"),
    );

    let review_stream_service = Arc::new(ReviewStreamService::new(
        push_repository,
        feed_user_repository,
        env.openai_config.api_key.clone(),
    ));

    let github_webhook_handler_state = GithubWebhookHandler::new(review_stream_service.clone());

    // Create auth service
    let user_repository = Arc::new(
        MongoUserRepository::new(&env.review_stream_config.mongo_url)
            .await
            .expect("Could not create push repository"),
    );
    let auth_service = Arc::new(AuthService::new(user_repository.clone()));

    // Create WebHtmxState
    let web_htmx_state = WebHtmxState {
        flash_config: axum_flash::Config::new(axum_flash::Key::generate()),
        review_feed_service: review_stream_service,
        auth_service,
        github_auth_config: web_htmx::state::GithubAuthConfig {
            client_id: env.github_auth_config.client_id.clone(),
            client_secret: env.github_auth_config.client_secret.clone(),
        },
    };

    let app = Router::new()
        .merge(web_routes(web_htmx_state))
        .merge(github_webhook_handler(github_webhook_handler_state))
        .route("/healthcheck", get(get_health_check));

    // Auth and session setup
    let client = Client::with_uri_str(&env.review_stream_config.mongo_url)
        .await
        .expect("Failed to create mongo client");
    let session_store = MongoDBStore::new(client, "sessions".to_string());
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(SessionManagerLayer::new(session_store.clone()).with_secure(false));

    let user_memory_store = MongoUserStore {
        users: user_repository.clone(),
    };

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax) // Ensure we send the cookie from the OAuth redirect.
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
