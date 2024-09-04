use checker::prefabs::num_check::NonZeroUnsignedError;
use persistence::ceobe_operate::release_version::Error as MongoError;
use serve_utils::{
    axum::extract::rejection::{JsonRejection, QueryRejection},
    status_err::StatusErr,
    ControllerError, ThisError,
};
use tencent_cloud_server::error::TcCloudError;

#[derive(Debug, ThisError, StatusErr)]
#[status_err(resp_err)]
pub enum Error {
    #[error(transparent)]
    Mongo(#[from] MongoError),
    #[error(transparent)]
    Query(#[from] QueryRejection),
    #[error(transparent)]
    Json(#[from] JsonRejection),
    #[error(transparent)]
    TencentCDN(#[from] TcCloudError),
    #[error(transparent)]
    InvalidPaginator(#[from]NonZeroUnsignedError)
}

impl ControllerError for crate::ReleaseVersionController {
    type Error = Error;
}
