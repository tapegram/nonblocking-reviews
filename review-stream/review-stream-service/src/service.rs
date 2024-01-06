use std::sync::Arc;

use openai_api_rs::v1::api::Client;

use crate::{
    get_feed::{GetFeed, GetFeedInput, GetFeedOutput},
    handle_github_push::{HandleGithubPush, HandleGithubPushInput, HandleGithubPushOutput},
    ports::push_repository::PushRepository,
};

#[derive(Clone)]
pub struct ReviewStreamService {
    //##PLOP INSERT COMMAND HOOK##
    pub get_feed: GetFeed,
    pub handle_github_push: HandleGithubPush,
}

impl ReviewStreamService {
    pub fn new(push_repository: Arc<dyn PushRepository>, openai_api_key: String) -> Self {
        let openai_client = Arc::new(Client::new(openai_api_key));
        Self {
            //##PLOP INSERT COMMAND INSTANTIATION HOOK##
            get_feed: GetFeed {
                push_repository: push_repository.clone(),
            },
            handle_github_push: HandleGithubPush {
                push_repository: push_repository.clone(),
                openai_client,
            },
        }
    }
    //##PLOP INSERT DELEGATE HOOK##
    pub async fn get_feed(&self, input: GetFeedInput) -> GetFeedOutput {
        self.get_feed.get_feed(input).await
    }

    pub async fn handle_github_push(&self, input: HandleGithubPushInput) -> HandleGithubPushOutput {
        self.handle_github_push.handle_github_push(input).await
    }
}
