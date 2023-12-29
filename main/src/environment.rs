use dotenvy::dotenv;
use tracing::instrument;

/**
* A single struct that represents all of the env vars.
* This struct should be created once during bootstrapping and then its values can be handed out as
* necessary
*/
pub struct Environment {
    github_app_config: GithubAppConfig,
    review_stream_config: ReviewStreamConfig,
}

pub struct GithubAppConfig {
    app_id: String,
    private_key_path: String,
    webhook_secret: String,
}

pub struct ReviewStreamConfig {
    mongo_url: String,
}

/**
* Function to do all the "dirty work" of pulling env vars into the Environment struct.
*/
#[instrument]
pub fn load_environment() -> Environment {
    dotenv().ok();
    Environment {
        github_app_config: GithubAppConfig {
            app_id: std::env::var("GITHUB_APP_ID").expect("GITHUB_APP_ID must be set"),
            private_key_path: std::env::var("GITHUB_PRIVATE_KEY_PATH")
                .expect("GITHUB_PRIVATE_KEY_PATH must be set"),
            webhook_secret: std::env::var("GITHUB_WEBHOOK_SECRET")
                .expect("GITHUB_WEBHOOK_SECRET must be set"),
        },
        review_stream_config: ReviewStreamConfig {
            mongo_url: std::env::var("MONGO_URL").expect("MONGO_URL must be set"),
        },
    }
}
