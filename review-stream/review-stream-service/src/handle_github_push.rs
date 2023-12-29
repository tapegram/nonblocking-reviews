use std::sync::Arc;

use octocrab::models::webhook_events::{
    payload::PushWebhookEventPayload, WebhookEvent, WebhookEventPayload, WebhookEventType,
};
use thiserror::Error;

use crate::{models::Push, ports::push_repository::PushRepository};

#[derive(Clone)]
pub struct HandleGithubPush {
    pub push_repository: Arc<dyn PushRepository>,
}

#[derive(Clone, Debug)]
pub struct HandleGithubPushInput {
    // Put input fields here
    // For lazy mapping reasons, just reusing the generic octocrab github event type and checking
    // its push at runtime. We could add more mapping and "parse dont validate" this into the
    // adapter.
    pub github_event: WebhookEvent,
}

// Change the return type, if needed
pub type HandleGithubPushOutput = Result<(), HandleGithubPushFailure>;

impl HandleGithubPush {
    pub async fn handle_github_push(&self, input: HandleGithubPushInput) -> HandleGithubPushOutput {
        let push_event: Box<PushWebhookEventPayload> = match input.github_event.specific {
            WebhookEventPayload::Push(push_event) => push_event,
            _ => return Err(HandleGithubPushFailure::NotAPushEvent),
        };

        todo!("")
    }
}

impl From<Box<PushWebhookEventPayload>> for Push {
    fn from(payload: Box<PushWebhookEventPayload>) -> Self {
        todo!("")
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum HandleGithubPushFailure {
    #[error("Not a push event")]
    NotAPushEvent,
    #[error("Something went wrong")]
    Unknown(String),
}
