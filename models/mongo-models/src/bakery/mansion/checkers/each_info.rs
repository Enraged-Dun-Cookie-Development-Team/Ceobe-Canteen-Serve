use checker::Checker;
use futures::future::{ready, Ready};

use super::{CheckError, MaxLimitString};
use crate::bakery::mansion::{checked::Info, preludes::Predict};

#[checker::check_gen(
    uncheck = EachInfoUncheck,
    checked = Info,
    error = CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct InfoChecker {
    #[serde(alias = "forecast_status")]
    pub predict: PredictLevelChecker,
    pub forecast: MaxLimitString<2048>,
}

pub struct PredictLevelChecker;

impl Checker for PredictLevelChecker {
    type Args = ();
    type Checked = Predict;
    type Err = CheckError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        use CheckError::UnknownPredictType;
        ready(match uncheck.as_str() {
            "false" => Ok(Predict::False),
            "unknown" => Ok(Predict::Unknown),
            "true" => Ok(Predict::True),
            _ => Err(UnknownPredictType(uncheck)),
        })
    }
}
