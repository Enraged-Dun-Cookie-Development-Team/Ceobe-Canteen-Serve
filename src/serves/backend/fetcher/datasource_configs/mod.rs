pub mod view;
use checker::{JsonCheckExtract, QueryCheckExtract};
use orm_migrate::sql_models::fetcher::datasource_config::checkers::FetcherDatasourceConfigChecker;
use page_size::request::PageSizeChecker;
use qiniu_cdn_upload::{
    update_payload::UploadPayload, update_source::FieldSource,
};
use uuid::Uuid;

use self::error::DatasourceConfigError;

mod controllers;
mod error;

type PageSizePretreatment =
    QueryCheckExtract<PageSizeChecker, DatasourceConfigError>;

/// 上传数据源验证
type FetcherDatasourceCheck =
    JsonCheckExtract<FetcherDatasourceConfigChecker, DatasourceConfigError>;

pub struct DataSourceAvatarPayload(String);

impl DataSourceAvatarPayload {
    pub fn new() -> Self { Self(Uuid::new_v4().to_string()) }
}

impl UploadPayload for DataSourceAvatarPayload {
    type Source = FieldSource;

    const DIR: &'static str = "datasource-avatar";

    fn obj_name(&self) -> &str { &self.0 }
}
