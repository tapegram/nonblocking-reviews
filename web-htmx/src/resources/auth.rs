use crate::state::WebHtmxState;
use crate::{components::page::PageLayout, routes};
use axum::extract::{Query, State};
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};

use rscx::{component, html, props};
use serde::Deserialize;
use web_client::server::form::Button;

pub fn auth_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::LOGIN, get(get_login))
        .route(routes::GITHUB_AUTH_CALLBACK, get(get_github_auth_callback))
        .with_state(state)
}

async fn get_login(State(state): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Login">
            <LoginForm github_client_id=state.github_auth_config.client_id />
        </PageLayout>
    })
}

#[props]
struct LoginFormProps {
    #[builder(setter(into))]
    github_client_id: String,
}

#[component]
fn LoginForm(props: LoginFormProps) -> String {
    html! {
        <a href=format!("https://github.com/login/oauth/authorize?scope=user:email&client_id={}", props.github_client_id)>
          Login with Github
        </a>
    }
}

#[derive(Deserialize)]
struct GithubAuthCallbackQueryParams {
    code: String,
}
async fn get_github_auth_callback(
    State(state): State<WebHtmxState>,
    Query(query_params): Query<GithubAuthCallbackQueryParams>,
) -> impl IntoResponse {
    let client = reqwest::Client::new();
    let params = &[
        ("client_id", state.github_auth_config.client_id.clone()),
        (
            "client_secret",
            state.github_auth_config.client_secret.clone(),
        ),
        ("code", query_params.code.clone()),
    ];

    let url =
        reqwest::Url::parse_with_params("https://github.com/login/oauth/access_token", params)
            .expect("Failed to parse params");
    let res = client
        .post(url)
        .header("Accept", "application/json")
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

    // We need to keep this token in the session of the user.
    // Before that, we need to decide if we are relying PURELY on github auth or do we need our own
    // user concept
    Html(html! {
        <p>{format!("{:?}", access_token_response)}</p>
    })
}
