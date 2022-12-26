use checker::QueryCheckExtract;
use page_size::request::PageSizeChecker;

use self::error::DatasourceConfigError;

mod view;
mod error;
mod controllers;

type PageSizePretreatment =
    QueryCheckExtract<PageSizeChecker, DatasourceConfigError>;