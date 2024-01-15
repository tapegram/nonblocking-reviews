use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: String,
    pub email: String,
    pub auth_id: String,
    pub subscriptions: Vec<RepositorySubscriptionRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositorySubscriptionRecord {
    pub id: String,
    pub external_id: String, // The github repository ID
    pub name: String,
}
