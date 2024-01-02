use std::{collections::HashMap, sync::Arc};

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
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

async fn handle_github_webhook(
    State(GithubWebhookHandler {
        review_stream_service,
        octocrab_client,
    }): State<GithubWebhookHandler>,
    request: Request,
) -> () {
    print!("Received a github webhook request {:?}", request);
    warn!("Received a github webhook request {:?}", request);
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

            let compare_url: String = push_event.as_ref().compare.to_string();
            let diff = reqwest::get(format!("{}.diff", &compare_url))
                .await
                .expect("Failed to get diff of the commit")
                .text()
                .await
                .expect("Failed to get diff of the commit");

            let mut ml_summary_json_body = HashMap::new();
            ml_summary_json_body.insert("body", diff.clone());

            #[derive(Serialize, Deserialize)]
            struct Summary {
                summary: GeneratedText,
            }
            #[derive(Serialize, Deserialize)]
            struct GeneratedText {
                generated_text: String,
            }
            let client = reqwest::Client::new();
            let summary_response: Summary = client
                .post("https://0w10jtv5s9.execute-api.us-east-1.amazonaws.com/prod/diffsummary")
                .json(&ml_summary_json_body)
                .header("x-api-key", "FdYYnnC4MnaMW3UaACfBx1b1QfaasTEh6z9v1RmJ") // This key should
                // be an env var
                .send()
                .await
                .expect("Failed to get summary of the commit")
                .json()
                .await
                .expect("Failed to get summary of the commit");

            review_stream_service
                .handle_github_push(HandleGithubPushInput {
                    github_event: *push_event,
                    repository,
                    diff,
                    summary: summary_response.summary.generated_text,
                })
                .await
                .expect("Failed to handle push webhook")
        }
        _ => warn!("Ignoring event {:?}", event),
    };
}

#[derive(Clone)]
pub struct GithubWebhookHandler {
    pub review_stream_service: Arc<ReviewStreamService>,
    pub octocrab_client: Octocrab,
}

pub fn github_webhook_handler(state: GithubWebhookHandler) -> Router {
    Router::new()
        .route("/github-webhook", post(handle_github_webhook))
        .with_state(state.clone())
}
