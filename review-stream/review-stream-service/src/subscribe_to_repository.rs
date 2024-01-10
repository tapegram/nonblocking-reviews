use std::sync::Arc;

use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::Deserialize;
use thiserror::Error;

use crate::{models::RepositorySubscription, ports::user_repository::UserRepository};

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
    pub user_github_access_token: String, // To fetch the repo
}

// Change the return type, if needed
pub type SubscribeToRepositoryOutput = Result<(), SubscribeToRepositoryFailure>;

impl SubscribeToRepository {
    pub async fn subscribe_to_repository(
        &self,
        input: SubscribeToRepositoryInput,
    ) -> SubscribeToRepositoryOutput {
        #[derive(Debug, Deserialize)]
        struct RepoInfo {
            id: String,
            private: bool,
            full_name: String,
            permissions: Permissions,
        }
        #[derive(Debug, Deserialize)]
        struct Permissions {
            pull: String,
        }

        let repo_info = reqwest::Client::new()
            // Expecting repository_name to be of the format {owner}/{repo}
            .get(format!(
                "https://api.github.com/repos/{}",
                input.repository_name
            ))
            .header(ACCEPT.as_str(), "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header(
                AUTHORIZATION.as_str(),
                format!("Bearer {}", &input.user_github_access_token),
            )
            .send()
            .await
            .map_err(|e| SubscribeToRepositoryFailure::Unknown(e.to_string()))?
            .json::<RepoInfo>()
            .await
            .map_err(|e| SubscribeToRepositoryFailure::Unknown(e.to_string()))?;

        let subscription = RepositorySubscription {
            id: uuid::Uuid::new_v4().to_string(),
            external_id: repo_info.id,
            name: repo_info.full_name,
        };

        let user = self
            .user_repository
            .get_user(input.user_id.clone())
            .await
            .map_err(|e| SubscribeToRepositoryFailure::Unknown(e.to_string()))?
            .ok_or(SubscribeToRepositoryFailure::NotFound)?;

        let user = user.add_subscription(subscription);

        self.user_repository
            .save(user)
            .await
            .map_err(|e| SubscribeToRepositoryFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum SubscribeToRepositoryFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
