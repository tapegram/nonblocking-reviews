use dotenvy::dotenv;
use tracing::instrument;

/**
* A single struct that represents all of the env vars.
* This struct should be created once during bootstrapping and then its values can be handed out as
* necessary
*/
pub struct Environment {
    pub github_app_config: GithubAppConfig,
    pub review_stream_config: ReviewStreamConfig,
    pub openai_config: OpenAIConfig,
}

pub struct GithubAppConfig {
    pub app_id: String,
    pub private_key_path: String,
    pub webhook_secret: String,
}

pub struct ReviewStreamConfig {
    pub mongo_url: String,
    pub ml_api_key: String,
}

pub struct OpenAIConfig {
    pub api_key: String,
    pub api_base: String,
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
            ml_api_key: std::env::var("ML_API_KEY").expect("ML_API_KEY must be set"),
        },
        openai_config: OpenAIConfig {
            api_key: std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
            api_base: std::env::var("OPENAI_API_BASE").expect("OPENAI_API_BASE must be set"),
        },
    }
}
