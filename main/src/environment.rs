use dotenvy::dotenv;
use tracing::instrument;

/**
* A single struct that represents all of the env vars.
* This struct should be created once during bootstrapping and then its values can be handed out as
* necessary
*/
pub struct Environment {
    pub review_stream_config: ReviewStreamConfig,
    pub openai_config: OpenAIConfig,
}

pub struct ReviewStreamConfig {
    pub mongo_url: String,
}

pub struct OpenAIConfig {
    pub api_key: String,
}

/**
* Function to do all the "dirty work" of pulling env vars into the Environment struct.
*/
#[instrument]
pub fn load_environment() -> Environment {
    dotenv().ok();
    Environment {
        review_stream_config: ReviewStreamConfig {
            mongo_url: std::env::var("MONGO_URL").expect("MONGO_URL must be set"),
        },
        openai_config: OpenAIConfig {
            api_key: std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
        },
    }
}
