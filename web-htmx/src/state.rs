use std::sync::Arc;

use axum::extract::FromRef;
use review_stream_service::service::ReviewStreamService;

#[derive(Clone)]
pub struct WebHtmxState {
    pub flash_config: axum_flash::Config,
    pub review_feed_service: Arc<ReviewStreamService>,
}

impl FromRef<WebHtmxState> for axum_flash::Config {
    fn from_ref(state: &WebHtmxState) -> axum_flash::Config {
        state.flash_config.clone()
    }
}
