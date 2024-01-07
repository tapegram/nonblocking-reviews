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
        .with_state(state)
}

async fn get_login(
    State(state): State<WebHtmxState>,
    Query(NextUrl { next }): Query<NextUrl>,
) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Login">
            <LoginForm github_client_id=state.github_auth_config.client_id next=next/>
        </PageLayout>
    })
}

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

#[props]
struct LoginFormProps {
    #[builder(setter(into))]
    github_client_id: String,

    #[builder(setter(into))]
    next: Option<String>,
}

#[component]
fn LoginForm(props: LoginFormProps) -> String {
    html! {
        <a href=format!("https://github.com/login/oauth/authorize?scope=user:email&client_id={}", props.github_client_id)>
          Login with Github
        </a>
    }
}
