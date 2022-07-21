use checker::{check_obj, Checker};
use futures_util::future::ready;

use super::{MansionDataCheckerError, MaxLimitString};
use crate::mansion_data::{checked::Info, preludes::Predict};

check_obj! {
    #[derive(Debug,serde::Deserialize)]
    pub struct EachInfoUncheck = InfoChecker > Info{
        #[serde(alias="forecast_status")]
        pub predict: PredictLevelChecker,
        pub forecast: MaxLimitString<2048>
    }
    err:MansionDataCheckerError
}

pub struct PredictLevelChecker;

impl Checker for PredictLevelChecker {
    type Args = ();
    type Checked = Predict;
    type Err = MansionDataCheckerError;
    type Fut = futures_util::future::Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        use MansionDataCheckerError::UnknownPredictType;
        ready(match uncheck.as_str() {
            "false" => Ok(Predict::False),
            "unknown" => Ok(Predict::Unknown),
            "true" => Ok(Predict::True),
            _ => Err(UnknownPredictType(uncheck)),
        })
    }
}
