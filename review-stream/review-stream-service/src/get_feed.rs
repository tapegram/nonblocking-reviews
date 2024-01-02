use std::sync::Arc;

use thiserror::Error;

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct GetFeed {
    // Put infra dependencies in this struct
    // Below is an example of a repo dependency
    // pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct GetFeedInput {
    // Put input fields here
    pub id: String
}

// Change the return type, if needed
pub type GetFeedOutput = Result<(), GetFeedFailure>;

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
