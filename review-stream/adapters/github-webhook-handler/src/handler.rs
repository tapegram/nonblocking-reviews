use std::{sync::Arc};

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
};
use openai_api_rs::v1::{
    api::Client,
    chat_completion::{
        ChatCompletionMessage, ChatCompletionRequest, ChatCompletionResponse, MessageRole,
    },
    common::GPT3_5_TURBO,
};
use review_stream_service::{
    handle_github_push::HandleGithubPushInput, service::ReviewStreamService,
};

use tracing::{info, warn};

async fn handle_github_webhook(
    State(GithubWebhookHandler {
        review_stream_service,
        openai_client: openaiClient,
    }): State<GithubWebhookHandler>,
    request: Request,
) {
    print!("Received a github webhook request {:?}", request);
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

            let system_prompt = "You are a helpful intern summarizing code diffs into concise helpful tweets for engineers to review.\n\nPlease summarize the following diffs as a short, concise, friendly tweet with a focus on describing the primary intent of the change.\n";
            let summary_request = ChatCompletionRequest::new(
                GPT3_5_TURBO.to_string(),
                vec![
                    ChatCompletionMessage {
                        role: MessageRole::system,
                        content: String::from(system_prompt),
                        name: None,
                        function_call: None,
                    },
                    ChatCompletionMessage {
                        role: MessageRole::user,
                        content: diff.clone(),
                        name: None,
                        function_call: None,
                    },
                ],
            );
            let summary_completion: ChatCompletionResponse = openaiClient
                .chat_completion(summary_request)
                .expect("Failed to get summary of the commit");

            info!("Summary response {:?}", summary_completion);

            let choices = summary_completion.choices;
            let summary_response = choices[0].message.content.clone().unwrap();

            review_stream_service
                .handle_github_push(HandleGithubPushInput {
                    github_event: *push_event,
                    repository,
                    diff,
                    summary: summary_response,
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
    pub openai_client: Arc<Client>,
}

impl GithubWebhookHandler {
    pub fn new(review_stream_service: Arc<ReviewStreamService>, openai_api_key: String) -> Self {
        Self {
            review_stream_service,
            openai_client: Arc::new(Client::new(openai_api_key)),
        }
    }
}

pub fn github_webhook_handler(state: GithubWebhookHandler) -> Router {
    Router::new()
        .route("/github-webhook", post(handle_github_webhook))
        .with_state(state.clone())
}
