use persistence::ceobe_operate::release_version::Error as MongoError;
use serve_utils::{
    axum::extract::rejection::QueryRejection, status_err::StatusErr,
    ControllerError, ThisError,
};

#[derive(Debug, ThisError, StatusErr)]
#[status_err(resp_err)]
pub enum Error {
    #[error(transparent)]
    MongoError(#[from] MongoError),
    #[error(transparent)]
    Query(#[from] QueryRejection),
}

impl ControllerError for crate::ReleaseVersionController {
    type Error = Error;
}
