
use checker::check_obj;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::fetcher::config::models::model_config;

#[derive(Debug, TypedBuilder)]
pub struct FetcherConfig {
    
}

#[check_obj(
    uncheck = FetcherConfigUncheck,
    checked = FetcherConfig,
    error = CheckError
)]
#[derive(Debug,serde::Deserialize)]
pub struct FetcherConfigChecker {
        
}

impl model_config::ActiveModel {
    
}
