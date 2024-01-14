use review_stream_service::models::{RepositorySubscription, User};

use crate::records::{RepositorySubscriptionRecord, UserRecord};

// Mapping from User to UserRecord
fn user_to_user_record(user: &User) -> UserRecord {
    let subscriptions = user
        .subscriptions
        .iter()
        .map(|sub| RepositorySubscriptionRecord {
            id: sub.id.clone(),
            external_id: sub.external_id.clone(),
            name: sub.name.clone(),
        })
        .collect();

    UserRecord {
        id: user.id.clone(),
        subscriptions,
        auth_id: user.auth_id.clone(),
    }
}

// Mapping from UserRecord to User
pub fn user_record_to_user(user_record: &UserRecord) -> User {
    let subscriptions = user_record
        .subscriptions
        .iter()
        .map(|sub| RepositorySubscription {
            id: sub.id.clone(),
            external_id: sub.external_id.clone(),
            name: sub.name.clone(),
        })
        .collect();

    User {
        id: user_record.id.clone(),
        subscriptions,
        auth_id: user_record.auth_id.clone(),
    }
}
