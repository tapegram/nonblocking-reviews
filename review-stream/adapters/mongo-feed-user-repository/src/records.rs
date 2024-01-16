use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: String,

    #[serde(default)]
    pub email: String,

    pub auth_id: String,

    #[serde(default)]
    pub subscriptions: Vec<RepositorySubscriptionRecord>,

    #[serde(default)]
    pub changes: Vec<RepositoryChangesRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositorySubscriptionRecord {
    pub id: String,
    pub external_id: String, // The github repository ID
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryChangesRecord {
    pub repository_id: String,
    pub push_id: String,
    pub changes: Vec<FileChangeRecord>,
    pub last_push: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileChangeRecord {
    pub path: String,
}
