use axum::extract::rejection::QueryRejection;
use persistence::ceobe_operate::release_version::Error;

use crate::error_generate;

error_generate! {
    pub CeobeOperationVersionError

    Query = QueryRejection
    MongoError = Error
}
