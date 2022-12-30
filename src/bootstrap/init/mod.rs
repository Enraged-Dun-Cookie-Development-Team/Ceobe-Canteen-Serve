use std::sync::Arc;

use axum_macros::FromRef;
use axum_starter::FromStateCollector;

use crate::serves::backend::ceobe_operation::video::QueryBiliVideo;

pub mod component_init;
pub mod db_init;
pub mod service_init;

#[derive(Debug, Clone, FromRef, FromStateCollector)]
pub struct State {
    qiniu: Arc<ceobe_qiniu_upload::Uploader>,
    bili: QueryBiliVideo,
}
