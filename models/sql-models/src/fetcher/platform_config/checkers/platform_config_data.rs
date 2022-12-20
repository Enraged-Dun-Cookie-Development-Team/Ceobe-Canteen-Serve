
use checker::check_obj;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::fetcher::platform_config::models::model_platform_config;

#[derive(Debug, TypedBuilder)]
pub struct FetcherPlatformConfig {
    
}

check_obj! {
    #[derive(Debug,serde::Deserialize)]
    pub struct FetcherPlatformConfigUncheck = FetcherPlatformConfigChecker > FetcherPlatformConfig{
        
    }
    err: CheckError
}

impl model_platform_config::ActiveModel {
    
}
