use checker::QueryCheckExtract;
use page_size::request::PageSizeChecker;

use self::error::PlatformConfigError;

mod controllers;
mod error;

type PageSizePretreatment =
    QueryCheckExtract<PageSizeChecker, PlatformConfigError>;