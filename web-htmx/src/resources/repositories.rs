use crate::{components::page::PageLayout, routes, state::WebHtmxState};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use axum_flash::Flash;
use http::{HeaderMap, StatusCode};
use rscx::{component, html, props};
use serde::Deserialize;
use web_client::server::{
    attrs::Attrs,
    form::{Button, GridCell, GridLayout, Label, TextInput},
    headers::SecondaryHeader,
    modal::{Modal, ModalSize},
};

pub fn repositories_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::REPOSITORIES, get(get_repositories))
        .with_state(state)
}

async fn get_repositories(State(state): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Repositories">
            {"Hello World"}
        </PageLayout>
    })
}
