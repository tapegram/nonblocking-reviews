use std::sync::Arc;

use thiserror::Error;

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct SubscribeToRepository {
    // Put infra dependencies in this struct
    // Below is an example of a repo dependency
    // pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct SubscribeToRepositoryInput {
    // Put input fields here
    pub id: String
}

// Change the return type, if needed
pub type SubscribeToRepositoryOutput = Result<(), SubscribeToRepositoryFailure>;

impl SubscribeToRepository {
    pub async fn subscribe_to_repository(&self, input: SubscribeToRepositoryInput) -> SubscribeToRepositoryOutput {
        todo!("Implement subscribe_to_repository")
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum SubscribeToRepositoryFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
