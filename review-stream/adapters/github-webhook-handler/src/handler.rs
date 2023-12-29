use axum::{extract::Request, routing::post, Router};
use http_body_util::BodyExt;
use octocrab::models::webhook_events::{WebhookEvent, WebhookEventType};
use tracing::{info, warn};

async fn handle_github_webhook(request: Request) -> () {
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
        WebhookEventType::Push => info!("Received a push event {:?}", event),
        _ => warn!("Ignored event {:?}", event),
    };
}

pub fn github_webhook_handler() -> Router {
    Router::new().route("/github-webhook", post(handle_github_webhook))
}
