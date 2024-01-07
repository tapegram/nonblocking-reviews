use auth_service::models::{User, UserPermission, UserRole};
use axum::{
    body::Body,
    extract::{Query, State},
    http::Request,
    middleware::Next,
    response::Response,
};
use axum_login::{tower_sessions::Session, AuthSession};
use mongo_user_repository::MongoUserStore;
use std::{collections::HashMap, future::Future};

use crate::state::WebHtmxState;

#[derive(Clone)]
pub struct Context {
    pub page_url: String,
    pub page_query_params: HashMap<String, String>,
    pub is_partial_request: bool,
    pub current_user: Option<LoggedInUser>,
}

#[derive(Clone)]
pub struct LoggedInUser {
    pub id: String,
    pub email: String,

    user: User,
}

tokio::task_local! {
    pub(crate) static CONTEXT: Context;
}

pub async fn provide_context_layer(
    State(state): State<WebHtmxState>,
    auth: AuthSession<MongoUserStore>,
    session: Session,
    request: Request<Body>,
    next: Next,
) -> Response {
    let Query(query_params): Query<HashMap<String, String>> =
        Query::try_from_uri(request.uri()).unwrap();

    let is_partial_request = request.headers().contains_key("Hx-Request");

    let current_user = match auth.user {
        Some(user) => Some(LoggedInUser {
            id: user.id.clone(),
            email: user.email.clone(),

            user,
        }),
        None => None,
    };

    let context = Context {
        page_url: request.uri().path().to_string(),
        page_query_params: query_params,
        is_partial_request,
        current_user,
    };

    // Set the context for this request.
    provide_context(context, next.run(request)).await
}

pub async fn provide_context<F: Future<Output = O>, O>(context: Context, f: F) -> O {
    CONTEXT.scope(context, f).await
}

pub fn context() -> Option<Context> {
    CONTEXT.try_with(|c| c.clone()).ok()
}
