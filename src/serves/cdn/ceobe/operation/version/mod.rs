mod controllers;
mod error;
mod models;

use axum_resp_result::MapReject;
use error::CeobeOperationVersionError as Error;
type Result<T> = core::result::Result<T, Error>;
use models::QueryReleaseVersion;

type MapRejecter<T> = MapReject<T, Error>;
