use std::sync::Arc;

use axum_macros::FromRef;
use axum_starter::FromStateCollector;
use ceobe_qiniu_upload::QiniuBaseUrl;
use fetcher_logic::axum_starter::starter_state::FetcherNotifyScheduleUrl;
use general_request_client::client::RequestClient;

pub mod component_init;
pub mod db_init;
pub mod service_init;

#[derive(Debug, Clone, FromRef, FromStateCollector)]
pub struct State {
    request_client: RequestClient,

    qiniu: Arc<ceobe_qiniu_upload::Uploader>,
    qiniu_base_url: QiniuBaseUrl,
    // fetcher request url
    fetcher_schedule_url: FetcherNotifyScheduleUrl,
}
