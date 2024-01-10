use std::sync::Arc;

use thiserror::Error;

use crate::ports::user_repository::UserRepository;

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct SubscribeToRepository {
    // Put infra dependencies in this struct
    // Below is an example of a repo dependency
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone, Debug)]
pub struct SubscribeToRepositoryInput {
    // Put input fields here
    pub repository_name: String, // Should be of the format {owner}/{repo}
    pub user_id: String,
}

// Change the return type, if needed
pub type SubscribeToRepositoryOutput = Result<(), SubscribeToRepositoryFailure>;

impl SubscribeToRepository {
    pub async fn subscribe_to_repository(
        &self,
        input: SubscribeToRepositoryInput,
    ) -> SubscribeToRepositoryOutput {
        // TODO: we should either ban subsribing to private repos entirely, or at least do a check
        // to see if the user should have access to a feed from this repository.
        todo!();
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum SubscribeToRepositoryFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
