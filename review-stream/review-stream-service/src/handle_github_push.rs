use std::sync::Arc;

use octocrab::models::webhook_events::payload::{PushWebhookEventCommit, PushWebhookEventPayload};
use openai_api_rs::v1::{
    api::Client,
    chat_completion::{
        ChatCompletionMessage, ChatCompletionRequest, ChatCompletionResponse, MessageRole,
    },
    common::GPT3_5_TURBO,
};
use thiserror::Error;
use tracing::info;

use crate::{
    models::{Author, Commit, Committer, Push, Pusher, Repository},
    ports::push_repository::PushRepository,
};

#[derive(Clone)]
pub struct HandleGithubPush {
    pub push_repository: Arc<dyn PushRepository>,
    pub openai_client: Arc<Client>,
}

#[derive(Clone, Debug)]
pub struct HandleGithubPushInput {
    // Put input fields here
    pub github_event: PushWebhookEventPayload,
    pub repository: octocrab::models::Repository,
    // Fetched from github commit api
    pub diff: String,
}

// Change the return type, if needed
pub type HandleGithubPushOutput = Result<(), HandleGithubPushFailure>;

impl HandleGithubPush {
    pub async fn handle_github_push(&self, input: HandleGithubPushInput) -> HandleGithubPushOutput {
        let summary = self
            .get_summary_from_openai(
                input.diff.clone(),
                input
                    .github_event
                    .head_commit
                    .clone()
                    .unwrap()
                    .message
                    .clone(),
            )
            .await;

        let push: Push = to_push(
            &input.github_event,
            &input.repository,
            &input.diff,
            &summary,
        );

        self.push_repository
            .save(push)
            .await
            .map_err(|e| HandleGithubPushFailure::Unknown(e.to_string()))?;

        Ok(())
    }

    /**
     * 1 - This should be moved behind a Summarizer abstraction
     * 2 - This should return a Result.
     */
    async fn get_summary_from_openai(&self, diff: String, msg: String) -> String {
        let system_prompt = r#"
            You are a helpful intern summarizing code diffs into concise helpful tweets for engineers to review.
            Please summarize the following diff (with a commit message as additional context) as a short, concise, friendly tweet with a focus on describing the primary intent of the change in one or two sentences.
        "#;
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
                    content: r#"
                        Diff: 
                        "#
                    .to_owned()
                        + &diff,
                    name: None,
                    function_call: None,
                },
                ChatCompletionMessage {
                    role: MessageRole::user,
                    content: r#"
                        Commit message: 
                        "#
                    .to_owned()
                        + &msg,
                    name: None,
                    function_call: None,
                },
            ],
        );
        let summary_completion: ChatCompletionResponse = self
            .openai_client
            .chat_completion(summary_request)
            .expect("Failed to get summary of the commit");

        info!("Summary response {:?}", summary_completion);

        let choices = summary_completion.choices;
        

        choices[0].message.content.clone().unwrap()
    }
}

fn to_push(
    payload: &PushWebhookEventPayload,
    repository: &octocrab::models::Repository,
    diff: &String,
    summary: &String,
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
        commits: payload.commits.iter().map(to_commit).collect::<Vec<_>>(),
        head_commit: to_commit(&payload.head_commit.clone().unwrap()),
        summary: summary.clone(),
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
