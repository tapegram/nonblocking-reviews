use crate::state::WebHtmxState;
use crate::{components::page::PageLayout, routes};
use axum::extract::Query;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};

use rscx::{component, html, props};
use serde::Deserialize;
use web_client::server::form::{Button, GridCell, GridLayout, Label, TextInput};

pub fn auth_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::LOGIN, get(get_login))
        .with_state(state)
}

async fn get_login(Query(NextUrl { next }): Query<NextUrl>) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Login">
            <LoginForm login_route=routes::login() next=next/>
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
    login_route: String,

    #[builder(setter(into))]
    next: Option<String>,
}

#[component]
fn LoginForm(props: LoginFormProps) -> String {
    html! {
        <form hx-post=props.login_route>
            <div class="pb-12">
                <p class="mt-1 text-sm leading-6 text-gray-600">
                    "pssst: try user@yallchart.com / password"
                </p>
                <GridLayout class="mt-10">
                    <GridCell span=4>
                        <Label for_input="email">Email</Label>
                        <TextInput input_type="email" name="email" autocomplete="email" />
                    </GridCell>
                    <GridCell span=4>
                        <Label for_input="password">Password</Label>
                        <TextInput input_type="password" name="password" autocomplete="password" />
                    </GridCell>
                    <GridCell span=4>
                        <div class="mt-6 flex items-center justify-end gap-x-6">
                            <Button kind="submit">Login</Button>
                        </div>
                    </GridCell>
                </GridLayout>
            </div>
            {
                match props.next {
                    Some(next) => html! {
                        <input type="hidden" name="next" value=next />
                    },
                    None => html! {},
                }
            }
        </form>
    }
}
