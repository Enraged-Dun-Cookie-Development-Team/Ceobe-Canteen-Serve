pub mod default_user;
pub mod init;
pub mod midllewares;

use std::sync::Arc;

use axum_macros::FromRef;
use axum_starter::FromStateCollector;
use ceobe_qiniu_upload::QiniuBaseUrl;
use general_request_client::client::RequestClient;
use mob_push_server::PartPushManagerState;
use qq_channel_warning::QqChannelGrpcState;
use request_clients::bili_client::QueryBiliVideo;
use scheduler_notifier::SchedulerUrl;

#[derive(Debug, Clone, FromRef, FromStateCollector)]
pub struct State {
    request_client: RequestClient,

    qiniu: Arc<ceobe_qiniu_upload::Manager>,
    bili: QueryBiliVideo,
    qiniu_base_url: QiniuBaseUrl,
    // fetcher request url
    scheduler_url: SchedulerUrl,
    mob_push: PartPushManagerState,
    qq_channel: QqChannelGrpcState,
}
