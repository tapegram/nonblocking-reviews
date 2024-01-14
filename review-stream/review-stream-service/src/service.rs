use std::sync::Arc;

use openai_api_rs::v1::api::Client;

use crate::{
    create_user::{CreateUser, CreateUserInput, CreateUserOutput},
    get_feed::{GetFeed, GetFeedInput, GetFeedOutput},
    get_user::{GetUser, GetUserInput, GetUserOutput},
    handle_github_push::{HandleGithubPush, HandleGithubPushInput, HandleGithubPushOutput},
    ports::{push_repository::PushRepository, user_repository::UserRepository},
    subscribe_to_repository::{
        SubscribeToRepository, SubscribeToRepositoryInput, SubscribeToRepositoryOutput,
    },
    unsubscribe_from_repository::{
        UnsubscribeFromRepository, UnsubscribeFromRepositoryInput, UnsubscribeFromRepositoryOutput,
    },
};

#[derive(Clone)]
pub struct ReviewStreamService {
    //##PLOP INSERT COMMAND HOOK##
    pub create_user: CreateUser,
    pub get_user: GetUser,
    pub unsubscribe_from_repository: UnsubscribeFromRepository,
    pub subscribe_to_repository: SubscribeToRepository,
    pub get_feed: GetFeed,
    pub handle_github_push: HandleGithubPush,
}

impl ReviewStreamService {
    pub fn new(
        push_repository: Arc<dyn PushRepository>,
        user_repository: Arc<dyn UserRepository>,
        openai_api_key: String,
    ) -> Self {
        let openai_client = Arc::new(Client::new(openai_api_key));
        Self {
            //##PLOP INSERT COMMAND INSTANTIATION HOOK##
            create_user: CreateUser {
                user_repository: user_repository.clone(),
            },
            get_user: GetUser {
                user_repository: user_repository.clone(),
            },
            unsubscribe_from_repository: UnsubscribeFromRepository {
              // Add any dependencies for the command here. They should be passed into this function and supplied by main.rs.
            },
            subscribe_to_repository: SubscribeToRepository {
                user_repository: user_repository.clone(),
            },
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
    pub async fn create_user(&self, input: CreateUserInput) -> CreateUserOutput {
        self.create_user.create_user(input).await
    }

    pub async fn get_user(&self, input: GetUserInput) -> GetUserOutput {
        self.get_user.get_user(input).await
    }

    pub async fn unsubscribe_from_repository(
        &self,
        input: UnsubscribeFromRepositoryInput,
    ) -> UnsubscribeFromRepositoryOutput {
        self.unsubscribe_from_repository
            .unsubscribe_from_repository(input)
            .await
    }

    pub async fn subscribe_to_repository(
        &self,
        input: SubscribeToRepositoryInput,
    ) -> SubscribeToRepositoryOutput {
        self.subscribe_to_repository
            .subscribe_to_repository(input)
            .await
    }

    pub async fn get_feed(&self, input: GetFeedInput) -> GetFeedOutput {
        self.get_feed.get_feed(input).await
    }

    pub async fn handle_github_push(&self, input: HandleGithubPushInput) -> HandleGithubPushOutput {
        self.handle_github_push.handle_github_push(input).await
    }
}
