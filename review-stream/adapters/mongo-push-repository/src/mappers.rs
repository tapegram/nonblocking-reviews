use review_stream_service::models::{Author, Commit, Committer, Push, Pusher, Repository};

use crate::records::{
    AuthorRecord, CommitRecord, CommitterRecord, PushRecord, PusherRecord, RepositoryRecord,
};

impl From<PushRecord> for Push {
    fn from(record: PushRecord) -> Self {
        record.to_push()
    }
}

impl PushRecord {
    fn to_push(&self) -> Push {
        Push {
            id: self.id.clone(),
            diff: self.diff.clone(),
            repository: self.repository.to_repository(),
            pusher: self.pusher.to_pusher(),
            compare_url: self.compare_url.clone(),
            commits: self.commits.iter().map(|c| c.to_commit()).collect(),
            head_commit: self.head_commit.to_commit(),
            branch_ref: self.branch_ref.clone(),
            summary: self.summary.clone(),
        }
    }
}

impl CommitRecord {
    fn to_commit(&self) -> Commit {
        Commit {
            added: self.added.clone(),
            author: self.author.to_author(),
            committer: self.committer.to_committer(),
            id: self.id.clone(),
            message: self.message.clone(),
            modified: self.modified.clone(),
            removed: self.removed.clone(),
            timestamp: self.timestamp.clone(),
            url: self.url.clone(),
        }
    }
}

impl RepositoryRecord {
    fn to_repository(&self) -> Repository {
        Repository {
            id: self.id.clone(),
            name: self.name.clone(),
            full_name: self.full_name.clone(),
            default_branch: self.default_branch.clone(),
        }
    }
}

impl PusherRecord {
    fn to_pusher(&self) -> Pusher {
        Pusher {
            name: self.name.clone(),
            email: self.email.clone(),
        }
    }
}

impl AuthorRecord {
    fn to_author(&self) -> Author {
        Author {
            name: self.name.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
        }
    }
}

impl CommitterRecord {
    fn to_committer(&self) -> Committer {
        Committer {
            name: self.name.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
        }
    }
}

pub fn to_push_record(push: &Push) -> PushRecord {
    PushRecord {
        id: push.id.clone(),
        diff: push.diff.clone(),
        repository: to_repository_record(&push.repository),
        pusher: to_pusher_record(&push.pusher),
        compare_url: push.compare_url.clone(),
        commits: push.commits.iter().map(|c| to_commit_record(c)).collect(),
        head_commit: to_commit_record(&push.head_commit),
        branch_ref: push.branch_ref.clone(),
        summary: push.summary.clone(),
    }
}

fn to_repository_record(repository: &Repository) -> RepositoryRecord {
    RepositoryRecord {
        id: repository.id.clone(),
        name: repository.name.clone(),
        full_name: repository.full_name.clone(),
        default_branch: repository.default_branch.clone(),
    }
}

fn to_commit_record(commit: &Commit) -> CommitRecord {
    CommitRecord {
        added: commit.added.clone(),
        author: to_author_record(&commit.author),
        committer: to_committer_record(&commit.committer),
        id: commit.id.clone(),
        message: commit.message.clone(),
        modified: commit.modified.clone(),
        removed: commit.removed.clone(),
        timestamp: commit.timestamp.clone(),
        url: commit.url.clone(),
    }
}

fn to_author_record(author: &Author) -> AuthorRecord {
    AuthorRecord {
        name: author.name.clone(),
        email: author.email.clone(),
        username: author.username.clone(),
    }
}

fn to_committer_record(committer: &Committer) -> CommitterRecord {
    CommitterRecord {
        name: committer.name.clone(),
        email: committer.email.clone(),
        username: committer.username.clone(),
    }
}

fn to_pusher_record(pusher: &Pusher) -> PusherRecord {
    PusherRecord {
        name: pusher.name.clone(),
        email: pusher.email.clone(),
    }
}
