use std::sync::Arc;

use axum_macros::FromRef;
use axum_starter::FromStateCollector;
use ceobe_qiniu_upload::QiniuBaseUrl;

pub mod component_init;
pub mod db_init;
pub mod service_init;

#[derive(Debug, Clone, FromRef, FromStateCollector)]
pub struct State {
    qiniu: Arc<ceobe_qiniu_upload::Uploader>,
    qiniu_base_url :QiniuBaseUrl
}
