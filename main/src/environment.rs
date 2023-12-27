

use dotenvy::dotenv;
use tracing::instrument;

/**
* A single struct that represents all of the env vars.
* This struct should be created once during bootstrapping and then its values can be handed out as
* necessary
*/
pub struct Environment {
    // Example:
    // pub mongo_db_url: String,
}

/**
* Function to do all the "dirty work" of pulling env vars into the Environment struct.
*/
#[instrument]
pub fn load_environment() -> Environment {
    dotenv().ok();
    Environment {
        // mongo_db_rl: env::var("MONGO_DB_URL").expect("MONGO_DB_URL must be set"),
    }
}
