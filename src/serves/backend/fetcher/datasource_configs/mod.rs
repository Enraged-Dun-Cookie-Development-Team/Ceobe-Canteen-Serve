use checker::{JsonCheckExtract, QueryCheckExtract};
use orm_migrate::sql_models::fetcher::{
    datasource_config::checkers::datasource_config_data::FetcherDatasourceConfigChecker,
};
use page_size::request::PageSizeChecker;

use self::error::DatasourceConfigError;

mod controllers;
mod error;
mod view;

type PageSizePretreatment =
    QueryCheckExtract<PageSizeChecker, DatasourceConfigError>;

// 上传数据源验证
type FetcherDatasourceCheck =
    JsonCheckExtract<FetcherDatasourceConfigChecker, DatasourceConfigError>;
