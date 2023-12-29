use async_trait::async_trait;
use chrono::{serde::ts_seconds, DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::doc;
use review_stream_service::{
    models::Push,
    ports::push_repository::{PushRepository, RepositoryFailure},
};

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
