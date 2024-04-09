use axum::extract::rejection::PathRejection;
use ceobe_cookie_logic::error::LogicError;

use crate::error_generate;

error_generate! {
    pub QiniuCdnDatasourceCombError

    Logic = LogicError
    Path = PathRejection
}
