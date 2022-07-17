use crate::{
    models::mansion::{check::Info, mongo_db::Predict},
    serves::frontend::bakery_mansion::error::{MansionError, UnknownPredictType},
    utils::{data_checker::DataChecker, data_struct::MaxLimitString},
};

crate::check_obj! {
    #[derive(Debug,serde::Deserialize)]
    // {#[derive(Debug,serde::Deserialize,serde::Serialize,Clone)]}
    pub struct EachInfoUncheck = InfoChecker > Info{
        #[serde(alias="forecast_status")]
        pub predict:PredictLevelChecker,
        pub forecast:MaxLimitString<2048>
    }
    err:MansionError
}

pub struct PredictLevelChecker;

impl DataChecker for PredictLevelChecker {
    type Args = ();
    type Checked = Predict;
    type Err = MansionError;
    type Fut = futures_util::future::Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let resp = match uncheck.as_str() {
            "false" => Ok(Predict::False),
            "unknown" => Ok(Predict::Unknown),
            "true" => Ok(Predict::True),
            _ => Err(UnknownPredictType.into()),
        };

        futures_util::future::ready(resp)
    }
}
