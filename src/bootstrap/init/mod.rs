use std::sync::Arc;

use axum_macros::FromRef;
use axum_starter::FromStateCollector;
use ceobe_qiniu_upload::QiniuBaseUrl;
use general_request_client::client::RequestClient;
use mob_push_server::{PushManager, PartPushManagerState};
use request_clients::bili_client::QueryBiliVideo;
use scheduler_notifier::SchedulerUrl;

pub mod component_init;
pub mod db_init;
pub mod service_init;

#[derive(Debug, Clone, FromRef, FromStateCollector)]
pub struct State {
    request_client: RequestClient,

    qiniu: Arc<ceobe_qiniu_upload::Manager>,
    bili: QueryBiliVideo,
    qiniu_base_url: QiniuBaseUrl,
    // fetcher request url
    scheduler_url: SchedulerUrl,
    mob_push: PartPushManagerState,
}
