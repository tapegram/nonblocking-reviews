use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Push {
    pub id: String,
    /**
     * The full git diff
     * https://stackoverflow.com/questions/40393117/getting-file-diff-with-github-api we probably want the diff (which is another request)
     *
     * Should this be a patch instead?
     */
    pub diff: String,
    pub repository: Repository,
    pub pusher: Pusher,

    pub compare_url: String,

    pub commits: Vec<Commit>,
    pub head_commit: Commit,
    pub branch_ref: String,

    pub summary: String, // This is the ML generated summary. This should probably be in a
                         // different record but we are just prototyping to learn AI stuff.
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub url: String,
    pub author: Author,
    pub committer: Committer,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Committer {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub full_name: String,
    pub default_branch: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pusher {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feed {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub summary: String,
    pub link: String,
    pub author: String,
    pub timestamp: DateTime<Utc>,
    pub repository: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub email: String, // This email is the email of the user in the auth service / github
    pub auth_id: String, // The id of the user in the Auth Service
    pub subscriptions: Vec<RepositorySubscription>,
    pub changes: Vec<RepositoryChanges>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryChanges {
    pub repository_id: String,
    pub push_id: String,
    pub changes: Vec<FileChange>,
    pub last_push: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChange {
    pub path: String,
}

impl User {
    pub fn new(id: String, auth_id: String, email: String) -> Self {
        Self {
            id,
            email,
            auth_id,
            subscriptions: vec![],
            changes: vec![],
        }
    }

    pub fn record_push_file_changes(&self, push: &Push) -> User {
        let collected_changes = push
            .commits
            .iter()
            .map(|commit| {
                let added = commit
                    .added
                    .iter()
                    .map(|path| FileChange { path: path.clone() })
                    .collect::<Vec<_>>();
                let removed = commit
                    .removed
                    .iter()
                    .map(|path| FileChange { path: path.clone() })
                    .collect::<Vec<_>>();
                let modified = commit
                    .modified
                    .iter()
                    .map(|path| FileChange { path: path.clone() })
                    .collect::<Vec<_>>();

                let changes = [added.as_slice(), removed.as_slice(), modified.as_slice()].concat();

                changes
            })
            .flatten()
            .collect::<Vec<_>>();

        let repository_changes = RepositoryChanges {
            repository_id: push.repository.id.clone(),
            push_id: push.id.clone(),
            changes: collected_changes,
            last_push: push.head_commit.timestamp.clone(),
        };

        User {
            changes: [vec![repository_changes].as_slice(), self.changes.as_slice()].concat(),
            ..self.clone()
        }
    }

    pub fn add_subscription(&self, subscription: RepositorySubscription) -> User {
        if self.subscriptions.iter().any(|s| s.id == subscription.id) {
            return self.clone();
        }

        User {
            subscriptions: [vec![subscription].as_slice(), self.subscriptions.as_slice()].concat(),
            ..self.clone()
        }
    }

    pub fn remove_subscription(&self, subscription_id: &String) -> User {
        User {
            subscriptions: self
                .subscriptions
                .iter()
                .filter(|s| &s.id != subscription_id)
                .cloned()
                .collect(),
            ..self.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositorySubscription {
    pub id: String,
    pub external_id: String, // The github repository ID
    pub name: String,
}
