use review_stream_service::models::{FileChange, RepositoryChanges, RepositorySubscription, User};

use crate::records::{
    FileChangeRecord, RepositoryChangesRecord, RepositorySubscriptionRecord, UserRecord,
};

// Mapping from User to UserRecord
pub fn user_to_user_record(user: &User) -> UserRecord {
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
        email: user.email.clone(),
        subscriptions,
        auth_id: user.auth_id.clone(),
        changes: user.changes.iter().map(|c| c.clone().into()).collect(),
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
        email: user_record.email.clone(),
        subscriptions,
        auth_id: user_record.auth_id.clone(),
        changes: user_record
            .changes
            .iter()
            .map(|c| c.clone().into())
            .collect(),
    }
}

impl From<RepositoryChanges> for RepositoryChangesRecord {
    fn from(changes: RepositoryChanges) -> Self {
        RepositoryChangesRecord {
            repository_id: changes.repository_id,
            push_id: changes.push_id,
            changes: changes
                .changes
                .into_iter()
                .map(FileChangeRecord::from)
                .collect(),
            last_push: changes.last_push,
        }
    }
}

impl From<FileChange> for FileChangeRecord {
    fn from(change: FileChange) -> Self {
        FileChangeRecord { path: change.path }
    }
}

impl From<RepositoryChangesRecord> for RepositoryChanges {
    fn from(changes: RepositoryChangesRecord) -> Self {
        RepositoryChanges {
            repository_id: changes.repository_id,
            push_id: changes.push_id,
            changes: changes.changes.into_iter().map(FileChange::from).collect(),
            last_push: changes.last_push,
        }
    }
}

impl From<FileChangeRecord> for FileChange {
    fn from(change: FileChangeRecord) -> Self {
        FileChange { path: change.path }
    }
}
