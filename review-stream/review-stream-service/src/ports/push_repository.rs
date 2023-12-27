use async_trait::async_trait;
use thiserror::Error;

use crate::models::Push;

#[async_trait]
pub trait PushRepository: Send + Sync + 'static {
    async fn get_push(&self, id: String) -> Result<Option<Push>, RepositoryFailure>;
    async fn save(&self, push: Push) -> Result<(), RepositoryFailure>;
}

#[derive(Error, Debug, PartialEq)]
pub enum RepositoryFailure {
    #[error("Failed to get connection from pool")]
    FailedToGetConnectionFromPool,
    #[error("Something went wrong")]
    Unknown(String),
}
