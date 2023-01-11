use checker::RefChecker;
use futures::future::{ready, Ready};
use sql_models::fetcher::config::checkers::config_data::FetcherConfig;

use crate::{error::LogicError, utils::TrueOrError};

pub struct PlatformSameChecker;

impl RefChecker for PlatformSameChecker {
    type Args = ();
    type Err = LogicError;
    type Fut = Ready<Result<(), Self::Err>>;
    type Target = Vec<FetcherConfig>;

    fn ref_checker(_: Self::Args, target: &Self::Target) -> Self::Fut {
        let mut iter = target
            .iter()
            .map(|FetcherConfig { platform, .. }| platform.as_str());

        ready('checker: {
            let Some(refer )= iter.next()else{
                break 'checker Ok(())
            };

            iter.all(|platform| refer == platform)
                .true_or_with(|| LogicError::PlatFromNotSame)
        })
    }
}
