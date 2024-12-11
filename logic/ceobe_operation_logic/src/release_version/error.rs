use std::convert::Infallible;

use persistence::ceobe_operate::release_version;
use status_err::ErrPrefix;
use tencent_cloud_server::error::TcCloudError;
use tokio::task::JoinError;

pub(super) type Rejection = Infallible;

#[derive(Debug, thiserror::Error, status_err::StatusErr)]
pub enum Error {
    #[error(transparent)]
    Dao(#[from] release_version::Error),
    #[error(transparent)]
    Tencent(#[from] TcCloudError),

    #[error(transparent)]
    #[status_err(err(prefix = "ErrPrefix::SERVE", err_code = 0x0003,))]
    JoinError(#[from] JoinError),

    #[error(transparent)]
    #[status_err(err = "transparent")]
    SerdeQs(#[from] serde_qs::Error),
}

pub(super) type LogicResult<T> = core::result::Result<T, Error>;
