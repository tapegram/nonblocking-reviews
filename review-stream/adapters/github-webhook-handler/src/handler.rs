use std::sync::Arc;

use axum::{
    extract::{Request, State},
    routing::post,
    Router,
};
use http_body_util::BodyExt;
use octocrab::{
    models::webhook_events::{
        payload::PushWebhookEventPayload, WebhookEvent, WebhookEventPayload, WebhookEventType,
    },
    Octocrab,
};
use review_stream_service::{
    handle_github_push::HandleGithubPushInput, service::ReviewStreamService,
};
use tracing::{info, warn};

async fn handle_github_webhook(
    State(GithubWebhookHandler {
        review_stream_service,
    }): State<GithubWebhookHandler>,
    request: Request,
) -> () {
    // request_from_github is the HTTP request your webhook handler received
    let (parts, body) = request.into_parts();
    let header = parts
        .headers
        .get("X-GitHub-Event")
        .unwrap()
        .to_str()
        .unwrap();

    let body = body.collect().await.unwrap().to_bytes();
    let event: WebhookEvent = WebhookEvent::try_from_header_and_body(header, &body).unwrap();
    // Now you can match on event type and call any specific handling logic
    match event.kind {
        WebhookEventType::Ping => info!("Received a ping"),
        WebhookEventType::PullRequest => info!("Received a pull request event"),
        WebhookEventType::Push => {
            info!("Received a push event {:?}", event);

            let push_event: Box<PushWebhookEventPayload> = match event.specific {
                WebhookEventPayload::Push(push_event) => push_event,
                _ => panic!("Expected push event"),
            };

            let repository: octocrab::models::Repository =
                event.repository.expect("Expected repository");

            review_stream_service
                .handle_github_push(HandleGithubPushInput {
                    github_event: *push_event,
                    repository,
                })
                .await
                .expect("Failed to handle push webhook")
        }
        _ => warn!("Ignored event {:?}", event),
    };
}

#[derive(Clone)]
pub struct GithubWebhookHandler {
    pub review_stream_service: Arc<ReviewStreamService>,
}

pub fn github_webhook_handler(state: GithubWebhookHandler) -> Router {
    Router::new()
        .route("/github-webhook", post(handle_github_webhook))
        .with_state(state.clone())
}
