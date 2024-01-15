use crate::{
    components::{
        page::{PageHeader, PageLayout},
        simple_form::{SimpleForm, SimpleFormData},
    },
    routes,
    state::WebHtmxState,
};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::{delete, get},
    Form, Router,
};
use axum_flash::Flash;
use futures::future::join_all;
use http::StatusCode;
use review_stream_service::{
    get_user::GetUserInput, subscribe_to_repository::SubscribeToRepositoryInput,
    unsubscribe_from_repository::UnsubscribeFromRepositoryInput,
};

use rscx::{component, html, props};
use serde::Deserialize;
use web_client::server::{
    button::PrimaryButton,
    headers::SecondaryHeader,
    modal::{modal_target, Modal},
    table::{ActionLink, TDVariant, Table, TableData, TableDataActions, TableHeading},
};

pub fn repositories_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::REPOSITORIES, get(get_repositories))
        .route(
            routes::REPOSITORIES_CREATE_FORM,
            get(get_create_form).post(post_create_form),
        )
        .route(routes::REPOSITORY, delete(delete_repository))
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
        <PageLayout
            header=PageHeader::Toolbar {
                title: "Subscribed Repositories".into(),
                buttons: html! {
                    <PrimaryButton
                        hx_get=routes::repositories_create_form()
                        hx_target=modal_target()
                        hx_swap="beforeend"
                        hx_push_url=routes::page_modal_from(routes::repositories_create_form())
                    >
                        Add Repository
                    </PrimaryButton>
                }
            }
        >
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
                                hx_delete=routes::repository(repo_subscription.id.clone())
                                sr_text="Remove repository subscription"
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

async fn get_create_form(State(_): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <PageLayout
            header="Add Repository Subscription"
        >
            <Modal>
                <SecondaryHeader
                    title="🏷️ Add Repository Subscription"
                    subtitle="Use the full name of the repository, e.g. `owner/repo`"
                />
                <RepositorySubscriptionForm
                    action=routes::repositories_create_form()
                />
            </Modal>
        </PageLayout>
    })
}

async fn post_create_form(
    State(WebHtmxState {
        review_feed_service,
        ..
    }): State<WebHtmxState>,
    flash: Flash,
    Form(form): Form<RepositorySubscriptionFormData>,
) -> impl IntoResponse {
    let ctx: crate::context::Context =
        crate::context::context().expect("Unable to retrieve htmx context.");
    let current_user = ctx.current_user.unwrap();

    review_feed_service
        .subscribe_to_repository(SubscribeToRepositoryInput {
            user_auth_id: current_user.id,
            user_github_access_token: current_user.access_token,
            repository_name: form.name,
        })
        .await
        .expect("Failed to add repository subscription");

    (
        StatusCode::OK,
        flash.success("Added new repository subscription!"),
        [
            ("hx-redirect", routes::repositories()),
            ("hx-retarget", "body".into()),
        ],
    )
}

#[derive(Deserialize, Debug, Default)]
struct RepositorySubscriptionFormData {
    name: String,
}

#[props]
struct RepositorySubscriptionFormProps {
    action: String,

    #[builder(default=RepositorySubscriptionFormData::default())]
    data: RepositorySubscriptionFormData,
}

#[component]
fn RepositorySubscriptionForm(props: RepositorySubscriptionFormProps) -> String {
    html! {
        <SimpleForm
            action=props.action
            data=SimpleFormData {
                name: props.data.name.clone(),
            }
        >
        </SimpleForm>
    }
}

async fn delete_repository(
    extract::Path(subscription_id): extract::Path<String>,
    State(WebHtmxState {
        review_feed_service,
        ..
    }): State<WebHtmxState>,
    flash: Flash,
) -> impl IntoResponse {
    let ctx: crate::context::Context =
        crate::context::context().expect("Unable to retrieve htmx context.");
    let current_user = ctx.current_user.unwrap();

    review_feed_service
        .unsubscribe_from_repository(UnsubscribeFromRepositoryInput {
            subscription_id: subscription_id.clone(),
            user_auth_id: current_user.id.clone(),
        })
        .await
        .expect("Failed to remove subscription");

    (
        StatusCode::OK,
        flash.success("Subscription removed!"),
        [
            ("hx-redirect", routes::repositories()),
            ("hx-retarget", "body".into()),
        ],
    )
}
