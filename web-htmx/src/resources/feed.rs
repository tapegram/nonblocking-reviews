use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use review_stream_service::get_feed::GetFeedInput;
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
                <GridCell span=5>
                    <Card class="m-4 p-4">
                            <GridLayout>
                                <GridCell>
                                    <a href={ item.link.clone() } class="text-lg">
                                        <p class="m-2">{ item.summary.clone() }</p>
                                    </a>
                                </GridCell>
                                <GridCell span=1>
                                    <a href=format!("https://github.com/{}", item.author.clone()) target="_blank" rel="noopener noreferrer">
                                        <p class="m-2 text-md">@{ item.author.clone() }</p>
                                    </a>
                                </GridCell>
                                <GridCell span=2>
                                    <a href=format!("https://github.com/{}", item.repository.clone()) target="_blank" rel="noopener noreferrer">
                                        <p class="m-2 text-md">{ item.repository.clone() }</p>
                                    </a>
                                </GridCell>
                            </GridLayout>
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
