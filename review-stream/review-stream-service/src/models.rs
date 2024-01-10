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
    pub auth_id: String, // The id of the user in the Auth Service
    pub subscriptions: Vec<RepositorySubscription>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositorySubscription {
    pub id: String,
    pub external_id: String, // The github repository ID
    pub name: String,
}
