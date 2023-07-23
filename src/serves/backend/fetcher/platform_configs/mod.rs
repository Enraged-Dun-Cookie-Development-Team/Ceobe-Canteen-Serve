use checker::{JsonCheckExtract, QueryCheckExtract};
use orm_migrate::sql_models::fetcher::platform_config::checkers::platform_config_data::FetcherPlatformConfigChecker;
use page_size::request::PageSizeChecker;

use self::error::PlatformConfigError;

mod controllers;
mod error;

/// 上传平台验证
type FetcherPlatformCheck =
    JsonCheckExtract<FetcherPlatformConfigChecker, PlatformConfigError>;

type PageSizePretreatment =
    QueryCheckExtract<PageSizeChecker, PlatformConfigError>;
