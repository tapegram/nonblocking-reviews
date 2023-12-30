use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use axum_flash::Flash;
use http::StatusCode;
use rscx::html;
use serde::Deserialize;

use web_client::server::{
    headers::SecondaryHeader,
    modal::{Modal, ModalSize},
};

use crate::{components::page::PageLayout, routes, state::WebHtmxState};

pub fn feed_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::FEED, get(get_feed))
        .with_state(state)
}

async fn get_feed(State(state): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <PageLayout>
            <p>"Hello world"</p>
        </PageLayout>
    })
}
