use async_trait::async_trait;

use mongodb::bson::doc;
use review_stream_service::{
    models::User,
    ports::user_repository::{RepositoryFailure, UserRepository},
};

use crate::{mappers::user_record_to_user, records::UserRecord};

#[derive(Clone, Debug)]
pub struct MongoFeedUserRepository {
    collection: mongodb::Collection<UserRecord>,
}

const DATABASE_NAME: &str = "review-stream";
const FEED_USERS_COLLECTION: &str = "feed-users"; // Named feed users to avoid conflicts with our
                                                  // identity users

impl MongoFeedUserRepository {
    pub fn from_client(client: &mongodb::Client) -> Result<Self, mongodb::error::Error> {
        let db = client.database(DATABASE_NAME);
        let collection = db.collection::<UserRecord>(FEED_USERS_COLLECTION);
        Ok(Self { collection })
    }
    pub async fn new(url: &String) -> Result<Self, mongodb::error::Error> {
        Ok(Self {
            collection: mongodb::Client::with_uri_str(url)
                .await?
                .database(DATABASE_NAME)
                .collection::<UserRecord>(FEED_USERS_COLLECTION),
        })
    }
}

#[async_trait]
impl UserRepository for MongoFeedUserRepository {
    async fn get_user(&self, _id: String) -> Result<Option<User>, RepositoryFailure> {
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
    async fn get_by_auth_id(&self, auth_id: String) -> Result<Option<User>, RepositoryFailure> {
        let filter = doc! { "auth_id": auth_id };
        let maybe_user = self
            .collection
            .find_one(filter, None)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        Ok(maybe_user.map(|u| user_record_to_user(&u)))
    }

    async fn save(&self, _user: User) -> Result<(), RepositoryFailure> {
        // let filter = doc! {"id": push.id.clone()};
        // let record = to_push_record(&push);
        // let options = mongodb::options::ReplaceOptions::builder()
        //     .upsert(true)
        //     .build();
        // self.collection
        //     .replace_one(filter, record, options)
        //     .await
        //     .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;
        // Ok(())
        todo!()
    }
}
