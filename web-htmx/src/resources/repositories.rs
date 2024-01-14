use crate::{components::page::PageLayout, routes, state::WebHtmxState};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures::future::join_all;
use review_stream_service::get_user::GetUserInput;

use rscx::html;
use web_client::server::{
    modal::modal_target,
    table::{ActionLink, TDVariant, Table, TableData, TableDataActions, TableHeading},
};

pub fn repositories_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::REPOSITORIES, get(get_repositories))
        .with_state(state)
}

async fn get_repositories(State(state): State<WebHtmxState>) -> impl IntoResponse {
    let ctx: crate::context::Context =
        crate::context::context().expect("Unable to retrieve htmx context.");
    let input = GetUserInput {
        auth_id: ctx.current_user.unwrap().id,
    };
    let user = state
        .review_feed_service
        .get_user(input)
        .await
        .expect("Failed to fetch user")
        .unwrap();

    Html(html! {
        <PageLayout header="Repositories">
            <Table
                headings=vec![
                    TableHeading::title("Name"),
                    TableHeading::empty("Actions"),
                ]
                body=join_all(user.subscriptions.iter().map(|repo_subscription| async { html! {
                    <TableData variant=TDVariant::First>{&repo_subscription.name}</TableData>
                    <TableData variant=TDVariant::Last>
                        <TableDataActions>
                            <ActionLink
                                hx_get=""//tag_edit_form(&props.worksite_id, &tag.id)
                                hx_target=modal_target()
                                hx_swap="beforeend"
                                hx_push_url="" // routes::page_modal_from(tag_edit_form(&props.worksite_id, &tag.id))
                                sr_text=&repo_subscription.name
                            >
                                Remove
                            </ActionLink>
                        </TableDataActions>
                    </TableData>
                }}))
                .await
            />
        </PageLayout>
    })
}
