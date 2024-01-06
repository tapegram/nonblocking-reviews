use axum::{
    extract::{State},
    response::{Html, IntoResponse},
    routing::get, Router,
};


use review_stream_service::{get_feed::GetFeedInput};
use rscx::{html, CollectFragmentAsync};


use web_client::server::{
    card::Card,
    form::{GridCell, GridLayout},
};

use crate::{
    components::{page::PageLayout, page_content::PageContent},
    routes,
    state::WebHtmxState,
};

pub fn feed_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::FEED, get(get_feed))
        .with_state(state)
}

async fn get_feed(State(state): State<WebHtmxState>) -> impl IntoResponse {
    let feed = state.review_feed_service.get_feed(GetFeedInput {}).await;

    let feed = match feed {
        Ok(feed) => feed,
        Err(e) => return Html(e.to_string()),
    };

    let content: String = feed
        .items
        .iter()
        .map(|item| async {
            html! {
                <GridCell>
                    <Card class="m-4 p-4">
                        <a href={ item.link.clone() } class="text-2xl">
                            <p class="m-2">@{ item.author.clone() }</p>
                            <p class="m-2">{ item.summary.clone() }</p>
                        </a>
                    </Card>
                </GridCell>
            }
        })
        .collect_fragment_async()
        .await;

    Html(html! {
        <PageLayout>
            <PageContent title="Code Feed">
                <GridLayout>
                    {content}
                </GridLayout>
            </PageContent>
        </PageLayout>
    })
}
