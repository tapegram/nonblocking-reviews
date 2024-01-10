

use thiserror::Error;

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct UnsubscribeFromRepository {
    // Put infra dependencies in this struct
    // Below is an example of a repo dependency
    // pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct UnsubscribeFromRepositoryInput {
    // Put input fields here
    pub id: String
}

// Change the return type, if needed
pub type UnsubscribeFromRepositoryOutput = Result<(), UnsubscribeFromRepositoryFailure>;

impl UnsubscribeFromRepository {
    pub async fn unsubscribe_from_repository(&self, _input: UnsubscribeFromRepositoryInput) -> UnsubscribeFromRepositoryOutput {
        todo!("Implement unsubscribe_from_repository")
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum UnsubscribeFromRepositoryFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
