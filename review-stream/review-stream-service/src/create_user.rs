use std::sync::Arc;

use thiserror::Error;

use crate::{models::User, ports::user_repository::UserRepository};

#[derive(Clone)]
pub struct CreateUser {
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone, Debug)]
pub struct CreateUserInput {
    // Put input fields here
    pub auth_id: String,
    pub email: String,
}

// Change the return type, if needed
pub type CreateUserOutput = Result<(), CreateUserFailure>;

impl CreateUser {
    pub async fn create_user(&self, input: CreateUserInput) -> CreateUserOutput {
        let maybe_existing_user = self
            .user_repository
            .get_by_auth_id(input.auth_id.clone())
            .await
            .map_err(|e| CreateUserFailure::Unknown(e.to_string()))?;

        // If the user already exists, do nothing and return early
        if maybe_existing_user.is_some() {
            return Ok(());
        }

        self.user_repository
            .save(User {
                id: uuid::Uuid::new_v4().to_string(),
                email: input.email,
                auth_id: input.auth_id,
                subscriptions: vec![],
            })
            .await
            .map_err(|e| CreateUserFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CreateUserFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
