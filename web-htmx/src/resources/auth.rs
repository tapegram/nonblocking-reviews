use crate::components::page_content::PageContent;
use crate::state::WebHtmxState;
use crate::{components::page::PageLayout, routes};
use auth_service::get_user_for_login::GetUserForLoginInput;
use axum::extract::{Query, State};
use axum::response::{Html, IntoResponse, Redirect};
use axum::{routing::get, Router};

use axum_login::AuthSession;
use http::StatusCode;
use mongo_user_repository::{Credentials, MongoUserStore};
use rscx::{component, html, props};
use serde::Deserialize;

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
    State(WebHtmxState {
        auth_service,
        github_auth_config,
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

    Redirect::to(routes::HOME).into_response()
}
