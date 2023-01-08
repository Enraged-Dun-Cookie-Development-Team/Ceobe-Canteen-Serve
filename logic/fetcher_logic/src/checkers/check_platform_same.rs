use checker::RefChecker;
use futures::future::{ready, Ready};
use sql_models::fetcher::config::checkers::config_data::FetcherConfig;

use crate::error::LogicError;

pub struct PlatformSameChecker;

impl RefChecker for PlatformSameChecker {
    type Target = Vec<FetcherConfig>;

    type Err = LogicError;

    type Args = ();

    type Fut = Ready<Result<(), Self::Err>>;

    fn ref_checker(_: Self::Args, target: &Self::Target) -> Self::Fut {
        let mut iter = target
            .iter()
            .map(|FetcherConfig { platform, .. }| platform.as_str());
            
        ready('checker: {
            let Some(refer )= iter.next()else{
                break 'checker Ok(())
            };

            if iter.all(|platform| refer == platform) {
                Ok(())
            } else {
                Err(LogicError::PlatFromNotSame)
            }
        })
    }
}
