use async_trait::async_trait;

use futures::TryStreamExt;
use mongodb::bson::doc;
use review_stream_service::{
    models::Push,
    ports::push_repository::{PushRepository, RepositoryFailure},
};
use tracing::info;

use crate::{mappers::to_push_record, records::PushRecord};

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
    async fn get_push(&self, _id: String) -> Result<Option<Push>, RepositoryFailure> {
        todo!("")
    }

    async fn get_all_pushes(
        &self,
        limit: i64,
        subscribed_repo_ids: Vec<String>,
    ) -> Result<Vec<Push>, RepositoryFailure> {
        info!("Getting all pushes");
        let filter =
            doc! { "summary": { "$exists": true }, "repository.id": { "$in": subscribed_repo_ids }};
        let options = mongodb::options::FindOptions::builder()
            .sort(doc! { "head_commit.timestamp": -1 })
            .limit(limit)
            .build();

        let cursor = self
            .collection
            .find(filter, options)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        let pushes: Vec<PushRecord> = cursor
            .try_collect()
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        Ok(pushes.iter().map(|p| p.to_push()).collect::<Vec<Push>>())
    }

    async fn save(&self, push: Push) -> Result<(), RepositoryFailure> {
        let filter = doc! {"id": push.id.clone()};
        let record = to_push_record(&push);
        let options = mongodb::options::ReplaceOptions::builder()
            .upsert(true)
            .build();
        self.collection
            .replace_one(filter, record, options)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;
        Ok(())
    }
}
