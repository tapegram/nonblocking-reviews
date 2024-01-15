use std::sync::Arc;

use thiserror::Error;

use crate::{
    models::Feed,
    ports::{push_repository::PushRepository, user_repository::UserRepository},
};

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct GetFeed {
    pub push_repository: Arc<dyn PushRepository>,
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone, Debug)]
pub struct GetFeedInput {
    pub auth_user_id: String,
}

// Change the return type, if needed
pub type GetFeedOutput = Result<Feed, GetFeedFailure>;

impl GetFeed {
    pub async fn get_feed(&self, input: GetFeedInput) -> GetFeedOutput {
        let user = self
            .user_repository
            .get_by_auth_id(input.auth_user_id)
            .await
            .map_err(|e| GetFeedFailure::Unknown(e.to_string()))?
            .ok_or(GetFeedFailure::NotFound)?;

        let pushes = self
            .push_repository
            .get_all_pushes(
                25,
                user.subscriptions
                    .iter()
                    .map(|s| s.external_id.clone())
                    .collect(),
            )
            .await
            .map_err(|e| GetFeedFailure::Unknown(e.to_string()))?;

        let items = pushes.into_iter().map(to_feed_item).collect();
        Ok(Feed { items })
    }
}

fn to_feed_item(push: crate::models::Push) -> crate::models::Item {
    crate::models::Item {
        timestamp: push.head_commit.timestamp,
        summary: push.summary,
        author: push.head_commit.author.username,
        link: push.compare_url,
        repository: push.repository.full_name,
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetFeedFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
