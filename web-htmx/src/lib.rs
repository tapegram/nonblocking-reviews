use axum::{
    middleware,
    response::{Html, Redirect},
    routing::get,
    Router,
};

use axum_login::login_required;
use http::StatusCode;
use mongo_user_repository::MongoUserStore;
use rscx::html;
use state::WebHtmxState;

use web_client::routes as client_routes;

//##PLOP USE RESOURCE HOOK##
use resources::repository::repository_routes;
use resources::repositories::repositories_routes;
use components::{not_found_message::NotFoundMessage, page::PageLayout};
use context::provide_context_layer;
use resources::auth::auth_routes;
use resources::feed::feed_routes;
use routes::{CLIENT, HOME, HOME_REDIRECT, PLAYGROUND};

pub mod components;
pub mod context;
pub mod livereload;
pub mod playground;
pub mod resources;
mod routes;
pub mod state;

pub fn routes(state: WebHtmxState) -> Router {
    Router::new()
        .with_state(state.clone())
        //##PLOP MERGE ROUTE HOOK##
.merge(repository_routes(state.clone()))
.merge(repositories_routes(state.clone()))
        .merge(feed_routes(state.clone()))
        .route(HOME, get(Redirect::temporary(HOME_REDIRECT)))
        // Anything above this RequireAuth route will require authentication
        .route_layer(login_required!(MongoUserStore, login_url = routes::login()))
        .merge(auth_routes(state.clone()))
        .nest(PLAYGROUND, playground::routes())
        .nest_service(CLIENT, client_routes())
        .fallback(fallback)
        .layer(middleware::from_fn_with_state(state, provide_context_layer))
}

async fn fallback() -> (StatusCode, Html<String>) {
    let not_found = html! {
        <PageLayout header="Oops!">
            <NotFoundMessage />
        </PageLayout>
    };

    (StatusCode::NOT_FOUND, Html(not_found))
}
