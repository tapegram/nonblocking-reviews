use crate::routes;
use crate::state::WebHtmxState;

use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use axum::{routing::get, Router};

use axum_login::AuthSession;
use http::header::USER_AGENT;
use http::StatusCode;
use mongo_user_repository::{Credentials, MongoUserStore};

use review_stream_service::create_user::CreateUserInput;
use serde::Deserialize;
use tracing::info;

pub fn auth_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::LOGIN, get(get_login))
        .route(routes::GITHUB_AUTH_CALLBACK, get(get_github_auth_callback))
        .with_state(state)
}

async fn get_login(State(state): State<WebHtmxState>) -> impl IntoResponse {
    Redirect::to(
        format!(
            "https://github.com/login/oauth/authorize?client_id={}",
            state.github_auth_config.client_id,
        )
        .as_str(),
    )
}

#[derive(Deserialize)]
struct GithubAuthCallbackQueryParams {
    code: String,
}

async fn get_github_auth_callback(
    State(WebHtmxState {
        github_auth_config,
        review_feed_service,
        ..
    }): State<WebHtmxState>,
    mut auth: AuthSession<MongoUserStore>,
    Query(query_params): Query<GithubAuthCallbackQueryParams>,
) -> impl IntoResponse {
    let client = reqwest::Client::new();
    let params = &[
        ("client_id", github_auth_config.client_id.clone()),
        ("client_secret", github_auth_config.client_secret.clone()),
        ("code", query_params.code.clone()),
    ];

    let url =
        reqwest::Url::parse_with_params("https://github.com/login/oauth/access_token", params)
            .expect("Failed to parse params");

    let res = client
        .post(url)
        .header("Accept", "application/json")
        .header(USER_AGENT.as_str(), "Code-Feed-App") // See: https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required
        .send()
        .await
        .expect("Failed to validate with github");

    #[derive(Deserialize, Debug)]
    struct GithubAccessTokenResponse {
        access_token: String,
        scope: String,
        token_type: String,
    }

    let access_token_response: GithubAccessTokenResponse = res
        .json()
        .await
        .expect("Failed to parse access token response");

    let creds = Credentials {
        access_code: access_token_response.access_token,
    };

    let user = match auth.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "Login failed").into_response(),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Login failed. Please try again.",
            )
                .into_response()
        }
    };

    if auth.login(&user).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Login failed. Please try again.",
        )
            .into_response();
    }

    info!("Creating user in review stream if they don't already exist");
    let input = CreateUserInput {
        auth_id: user.id.clone(),
        email: user.email.clone(),
    };

    review_feed_service
        .create_user(input)
        .await
        .expect("Failed to create user in review service");

    info!("Logged in!");

    Redirect::to(routes::HOME).into_response()
}
