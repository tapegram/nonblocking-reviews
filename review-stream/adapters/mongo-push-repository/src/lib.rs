use async_trait::async_trait;
use chrono::{serde::ts_seconds, DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::doc;
use review_stream_service::{
    models::Push,
    ports::push_repository::{PushRepository, RepositoryFailure},
};
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

#[derive(Clone, Debug)]
pub struct MongoPushRepository {
    collection: mongodb::Collection<PushRecord>,
}

const DATABASE_NAME: &str = "review-stream";
const PUSHES_COLLECTION: &str = "pushes";

impl MongoPushRepository {
    pub fn from_client(client: &mongodb::Client) -> Result<Self, mongodb::error::Error> {
        let db = client.database(DATABASE_NAME);
        let collection = db.collection::<PushRecord>(PUSHES_COLLECTION);
        Ok(Self { collection })
    }
    pub async fn new(url: &String) -> Result<Self, mongodb::error::Error> {
        Ok(Self {
            collection: mongodb::Client::with_uri_str(url)
                .await?
                .database(DATABASE_NAME)
                .collection::<PushRecord>(PUSHES_COLLECTION),
        })
    }
}

#[async_trait]
impl PushRepository for MongoPushRepository {
    async fn get_push(&self, id: String) -> Result<Option<Push>, RepositoryFailure> {
        // let filter = doc! { "id": id };
        // let maybe_worksite = self
        //     .collection
        //     .find_one(filter, None)
        //     .await
        //     .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;
        //
        // Ok(maybe_worksite.map(|w| w.to_worksite()))
        todo!("")
    }

    async fn save(&self, push: Push) -> Result<(), RepositoryFailure> {
        // let filter = doc! {"id": worksite.id.clone()};
        // let record = to_worksite_record(&worksite);
        // let options = mongodb::options::ReplaceOptions::builder()
        //     .upsert(true)
        //     .build();
        // self.collection
        //     .replace_one(filter, record, options)
        //     .await
        //     .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;
        // Ok(())
        todo!("")
    }
}
