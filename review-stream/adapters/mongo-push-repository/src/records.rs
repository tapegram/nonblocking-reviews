use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PushRecord {
    pub id: String,
    /**
     * The full git diff
     * https://stackoverflow.com/questions/40393117/getting-file-diff-with-github-api we probably want the diff (which is another request)
     *
     * Should this be a patch instead?
     */
    pub diff: String,
    pub repository: RepositoryRecord,
    pub pusher: PusherRecord,

    pub compare_url: String,

    pub commits: Vec<CommitRecord>,
    pub head_commit: CommitRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitRecord {
    pub timestamp: String,
    pub url: String,
    pub author: Author,
    pub committer: Committer,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Committer {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryRecord {
    pub id: String,
    pub name: String,
    pub full_name: String,
    pub default_branch: String,
    pub master_branch: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PusherRecord {
    pub name: String,
    pub email: String,
}
