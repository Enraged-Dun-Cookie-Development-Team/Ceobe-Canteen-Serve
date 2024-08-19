use persistence::ceobe_operate::release_version::Error as MongoError;
use serve_utils::{
    axum::extract::rejection::QueryRejection, status_err::StatusErr,
    ControllerError, ThisError,
};
use serve_utils::axum::extract::rejection::JsonRejection;

#[derive(Debug, ThisError, StatusErr)]
#[status_err(resp_err)]
pub enum Error {
    #[error(transparent)]
    MongoError(#[from] MongoError),
    #[error(transparent)]
    Query(#[from] QueryRejection),
    #[error(transparent)]
    Json(#[from]JsonRejection)
}

impl ControllerError for crate::ReleaseVersionController {
    type Error = Error;
}
