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
            master_branch: self.master_branch.clone(),
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
