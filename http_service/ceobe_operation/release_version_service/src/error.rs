use checker::prefabs::num_check::NonZeroUnsignedError;
use serve_utils::{
    axum::extract::rejection::{JsonRejection, QueryRejection},
    status_err::StatusErr,
    ControllerError, ThisError,
};

#[derive(Debug, ThisError, StatusErr)]
#[status_err(resp_err)]
pub enum Error {
    #[error(transparent)]
    Query(#[from] QueryRejection),
    #[error(transparent)]
    Json(#[from] JsonRejection),
    #[error(transparent)]
    InvalidPaginator(#[from] NonZeroUnsignedError),
    #[error(transparent)]
    Logic(#[from] ceobe_operation_logic::release_version::Error),
}

impl ControllerError for crate::ReleaseVersionController {
    type Error = Error;
}
