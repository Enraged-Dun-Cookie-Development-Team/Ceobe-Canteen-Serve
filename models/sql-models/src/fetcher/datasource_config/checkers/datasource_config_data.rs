
use checker::check_obj;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::fetcher::datasource_config::models::model_datasource_config;

#[derive(Debug, TypedBuilder)]
pub struct FetcherDatasourceConfig {
    
}

#[check_obj(
    uncheck = FetcherDatasourceConfigUncheck,
    checked = FetcherDatasourceConfig,
    error = CheckError
)]
#[derive(Debug,serde::Deserialize)]
pub struct FetcherDatasourceConfigChecker {
        
}

impl model_datasource_config::ActiveModel {
    
}
