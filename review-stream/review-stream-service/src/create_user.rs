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

        // Unwrap or create a new user
        let user = maybe_existing_user.unwrap_or_else(|| User {
            id: uuid::Uuid::new_v4().to_string(),
            email: input.email.clone(),
            auth_id: input.auth_id.clone(),
            subscriptions: vec![],
            changes: vec![],
        });
        // Make sure we always update the email (old data may not have an email address).
        let user = user.set_email(input.email.clone());

        self.user_repository
            .save(user)
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
