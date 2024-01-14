use std::sync::Arc;

use thiserror::Error;

use crate::{models::User, ports::user_repository::UserRepository};

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct GetUser {
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone, Debug)]
pub struct GetUserInput {
    // Put input fields here
    pub auth_id: String,
}

// Change the return type, if needed
pub type GetUserOutput = Result<Option<User>, GetUserFailure>;

impl GetUser {
    pub async fn get_user(&self, input: GetUserInput) -> GetUserOutput {
        let user = self
            .user_repository
            .get_by_auth_id(input.auth_id)
            .await
            .map_err(|e| GetUserFailure::Unknown(e.to_string()))?;

        Ok(user)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetUserFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
