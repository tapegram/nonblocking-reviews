use std::sync::Arc;

use thiserror::Error;

use crate::ports::user_repository::UserRepository;

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct UnsubscribeFromRepository {
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone, Debug)]
pub struct UnsubscribeFromRepositoryInput {
    pub subscription_id: String,
    pub user_auth_id: String,
}

// Change the return type, if needed
pub type UnsubscribeFromRepositoryOutput = Result<(), UnsubscribeFromRepositoryFailure>;

impl UnsubscribeFromRepository {
    pub async fn unsubscribe_from_repository(
        &self,
        input: UnsubscribeFromRepositoryInput,
    ) -> UnsubscribeFromRepositoryOutput {
        let user = self
            .user_repository
            .get_by_auth_id(input.user_auth_id.clone())
            .await
            .map_err(|e| UnsubscribeFromRepositoryFailure::Unknown(e.to_string()))?
            .ok_or(UnsubscribeFromRepositoryFailure::NotFound)?;

        let user = user.remove_subscription(&input.subscription_id);

        self.user_repository
            .save(user)
            .await
            .map_err(|e| UnsubscribeFromRepositoryFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum UnsubscribeFromRepositoryFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
