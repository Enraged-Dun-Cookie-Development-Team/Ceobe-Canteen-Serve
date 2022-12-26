use checker::{QueryCheckExtract, JsonCheckExtract};
use orm_migrate::sql_models::fetcher::{config::checkers::config_data::FetcherConfigChecker, datasource_config::checkers::datasource_config_data::FetcherDatasourceConfigChecker};
use page_size::request::PageSizeChecker;

use self::error::DatasourceConfigError;

mod view;
mod error;
mod controllers;

type PageSizePretreatment =
    QueryCheckExtract<PageSizeChecker, DatasourceConfigError>;

// 上传数据源验证
type FetcherDatasourceCheck = JsonCheckExtract<
    FetcherDatasourceConfigChecker,
    DatasourceConfigError,
>;