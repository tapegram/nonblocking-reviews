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
        .route(
            routes::REPOSITORIES_CREATE_FORM,
            get(get_create_form).post(post_create_form),
        )
        .with_state(state)
}

async fn get_repositories(State(state): State<WebHtmxState>) -> impl IntoResponse {
    todo!()
}

async fn get_create_form(headers: HeaderMap) -> impl IntoResponse {
    Html(html! {
        <PageLayout
            header="Add Repositories"
        >
            <Modal size=ModalSize::MediumScreen>
                <SecondaryHeader
                    title="Add Repository"
                    subtitle="Enter details below."
                />
                <RepositoryForm action=routes::repositories_create_form() />
            </Modal>
        </PageLayout>
    })
}

#[props]
pub struct RepositoryFormProps {
    #[builder(setter(into))]
    action: String,

    #[builder(setter(into), default)]
    name: String,
}

#[component]
pub fn RepositoryForm(props: RepositoryFormProps) -> String {
    html! {
        <form hx-post=props.action>
            <div class="pb-12">
                <GridLayout class="mt-10">
                    <GridCell span=3>
                        <Label for_input="name">Name</Label>
                        <TextInput name="name" input_type="name" value=props.name/>
                    </GridCell>
                </GridLayout>
            </div>
            <div class="mt-6 flex items-center justify-end gap-x-6">
                <Button
                    onclick="history.go(-1)"
                    attrs=Attrs::with("data-toggle-action", "close".into())
                >
                    Cancel
                </Button>
                <Button kind="submit">Save</Button>
            </div>
        </form>
    }
}

#[derive(Deserialize, Debug)]
struct AddRepositoryFormData {
    name: String,
}

async fn post_create_form(
    State(state): State<WebHtmxState>,
    flash: Flash,
    Form(form): Form<AddRepositoryFormData>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        flash.success("Repository added successfully!"),
        [
            ("hx-redirect", routes::repositories()),
            ("hx-retarget", "body".into()),
        ],
    )
}
