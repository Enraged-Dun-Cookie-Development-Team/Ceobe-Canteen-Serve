use axum_resp_result::RespResult;
use checker::{JsonCheckExtract, QueryCheckExtract};
use page_size::request::PageSizeChecker;
use persistence::admin::models::username::Checker;

use self::error::AdminUserError;

mod controllers;
mod error;
mod view;

type AdminUserRResult<T> = RespResult<T, error::AdminUserError>;

type UsernamePretreatment = JsonCheckExtract<Checker, AdminUserError>;

type PageSizePretreatment =
    QueryCheckExtract<PageSizeChecker, AdminUserError>;
