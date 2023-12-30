use std::sync::Arc;

use octocrab::models::webhook_events::{
    payload::{PushWebhookEventCommit, PushWebhookEventPayload},
    WebhookEvent, WebhookEventPayload,
};
use thiserror::Error;

use crate::{
    models::{Author, Commit, Committer, Push, Pusher, Repository},
    ports::push_repository::PushRepository,
};

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

        let repository: octocrab::models::Repository = input
            .github_event
            .repository
            .ok_or(HandleGithubPushFailure::NoRepository)?;

        let push: Push = to_push(push_event.as_ref(), &repository);

        self.push_repository
            .save(push)
            .await
            .map_err(|e| HandleGithubPushFailure::Unknown(e.to_string()))?;
        Ok(())
    }
}

fn to_push(payload: &PushWebhookEventPayload, repository: &octocrab::models::Repository) -> Push {
    Push {
        id: uuid::Uuid::new_v4().to_string(), // This dependency should be hoisted out
        diff: "".into(), // This should be passed in as well. Requires an extra query to github
        repository: to_repository(repository),
        pusher: Pusher {
            name: payload.pusher.user.name.clone(),
            email: payload.pusher.user.email.clone(),
        },
        compare_url: payload.compare.clone().to_string(),
        commits: payload
            .commits
            .iter()
            .map(|c| to_commit(c))
            .collect::<Vec<_>>(),
        head_commit: to_commit(&payload.head_commit.clone().unwrap()),
    }
}

fn to_commit(commit: &PushWebhookEventCommit) -> Commit {
    Commit {
        id: commit.id.clone(),
        message: commit.message.clone(),
        timestamp: commit.timestamp,
        url: commit.url.clone().into(),
        author: Author {
            name: commit.author.user.name.clone(),
            email: commit.author.user.email.clone(),
            username: commit.author.username.clone().unwrap(),
        },
        committer: Committer {
            name: commit.committer.user.name.clone(),
            email: commit.committer.user.email.clone(),
            username: commit.committer.username.clone().unwrap(),
        },
        added: commit.added.clone(),
        removed: commit.removed.clone(),
        modified: commit.modified.clone(),
    }
}

fn to_repository(repository: &octocrab::models::Repository) -> Repository {
    Repository {
        id: repository.id.to_string(),
        name: repository.name.clone(),
        full_name: repository.full_name.clone().unwrap_or("".into()),
        default_branch: repository.default_branch.clone().unwrap_or("".into()),
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum HandleGithubPushFailure {
    #[error("We need a repository to associate the push with")]
    NoRepository,
    #[error("Not a push event")]
    NotAPushEvent,
    #[error("Something went wrong")]
    Unknown(String),
}
