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
use http_body_util::BodyExt;

#[derive(Clone)]
pub struct HandleGithubPush {
    pub push_repository: Arc<dyn PushRepository>,
    pub octocrab_client: octocrab::Octocrab,
}

#[derive(Clone, Debug)]
pub struct HandleGithubPushInput {
    // Put input fields here
    pub github_event: PushWebhookEventPayload,
    pub repository: octocrab::models::Repository,
    pub diff: String,
}

// Change the return type, if needed
pub type HandleGithubPushOutput = Result<(), HandleGithubPushFailure>;

impl HandleGithubPush {
    pub async fn handle_github_push(&self, input: HandleGithubPushInput) -> HandleGithubPushOutput {
        let push: Push = to_push(&input.github_event, &input.repository, &input.diff);

        self.push_repository
            .save(push)
            .await
            .map_err(|e| HandleGithubPushFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

fn to_push(
    payload: &PushWebhookEventPayload,
    repository: &octocrab::models::Repository,
    diff: &String,
) -> Push {
    Push {
        id: uuid::Uuid::new_v4().to_string(), // This dependency should be hoisted out
        branch_ref: payload.r#ref.clone(),
        diff: diff.into(), // This should be passed in as well. Requires an extra query to github
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
