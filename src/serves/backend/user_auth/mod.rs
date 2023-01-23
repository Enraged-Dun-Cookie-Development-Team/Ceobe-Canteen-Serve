use checker::{JsonCheckExtract, QueryCheckExtract};
use orm_migrate::sql_models::admin_user::username::Checker;
use page_size::request::PageSizeChecker;
use resp_result::RespResult;

use self::error::AdminUserError;

mod controllers;
mod error;
mod view;

type AdminUserRResult<T> = RespResult<T, error::AdminUserError>;

type UsernamePretreatment = JsonCheckExtract<Checker, AdminUserError>;

type PageSizePretreatment =
    QueryCheckExtract<PageSizeChecker, AdminUserError>;
