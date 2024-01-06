use std::sync::Arc;

use thiserror::Error;

use crate::{models::Feed, ports::push_repository::PushRepository};

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct GetFeed {
    // Put infra dependencies in this struct
    pub push_repository: Arc<dyn PushRepository>,
}

#[derive(Clone, Debug)]
pub struct GetFeedInput {}

// Change the return type, if needed
pub type GetFeedOutput = Result<Feed, GetFeedFailure>;

impl GetFeed {
    pub async fn get_feed(&self, _input: GetFeedInput) -> GetFeedOutput {
        let pushes = self
            .push_repository
            .get_all_pushes(25)
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
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetFeedFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
