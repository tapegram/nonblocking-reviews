use crate::{components::page::PageLayout, routes, state::WebHtmxState};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get, Router,
};


use rscx::{html};



pub fn repositories_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::REPOSITORIES, get(get_repositories))
        .with_state(state)
}

async fn get_repositories(State(_state): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Repositories">
            {"Hello World"}
        </PageLayout>
    })
}
