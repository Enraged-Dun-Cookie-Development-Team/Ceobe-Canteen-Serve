use checker::{Checker, check_obj};
use range_limit::{RangeBoundLimit, limits::max_limit::MaxLimit};

use crate::mansion::{check::Info, MansionDataError, preludes::Predict};

type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;
check_obj! {
    #[derive(Debug,serde::Deserialize)]
    // {#[derive(Debug,serde::Deserialize,serde::Serialize,Clone)]}
    pub struct EachInfoUncheck = InfoChecker > Info{
        #[serde(alias="forecast_status")]
        pub predict:PredictLevelChecker,
        pub forecast:MaxLimitString<2048>
    }
    err:MansionDataError
}

pub struct PredictLevelChecker;

impl Checker for PredictLevelChecker {
    type Args = ();
    type Checked = Predict;
    type Err = MansionDataError;
    type Fut = futures_util::future::Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let resp = match uncheck.as_str() {
            "false" => Ok(Predict::False),
            "unknown" => Ok(Predict::Unknown),
            "true" => Ok(Predict::True),
            _ => Err(MansionDataError::UnknownPredictType.into()),
        };

        futures_util::future::ready(resp)
    }
}
