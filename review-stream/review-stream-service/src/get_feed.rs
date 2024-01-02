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
    pub async fn get_feed(&self, input: GetFeedInput) -> GetFeedOutput {
        todo!("Implement get_feed")
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetFeedFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
