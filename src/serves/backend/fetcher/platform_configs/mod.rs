use checker::{JsonCheckExtract, QueryCheckExtract};
use page_size::request::PageSizeChecker;
use persistence::fetcher::models::platform_config::checkers::platform_config_data::FetcherPlatformConfigChecker;

use self::error::PlatformConfigError;

mod controllers;
mod error;

/// 上传平台验证
type FetcherPlatformCheck =
    JsonCheckExtract<FetcherPlatformConfigChecker, PlatformConfigError>;

type PageSizePretreatment =
    QueryCheckExtract<PageSizeChecker, PlatformConfigError>;
