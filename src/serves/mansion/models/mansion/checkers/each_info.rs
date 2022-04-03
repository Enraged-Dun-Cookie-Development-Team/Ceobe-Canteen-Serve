use crate::{
    serves::mansion::error::{MansionError, UnknownPredictType},
    utils::{data_checker::DataChecker, data_struct::MaxLimitString},
};

crate::check_obj! {
    {#[derive(Debug,serde::Deserialize)]}
    {#[derive(Debug,serde::Deserialize,serde::Serialize,Clone)]}
    pub struct EachInfoUncheck =InfoChecker > Info{
        #[serde(alias="isTrue")]
        pub predict:PredictLevelChecker,
        pub forecast:MaxLimitString<2048>
    }
    err:MansionError
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Predict {
    #[serde(rename = "false")]
    False,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "true")]
    True,
}

pub struct PredictLevelChecker;

impl DataChecker for PredictLevelChecker {
    type Args = ();
    type Checked = Predict;
    type Err = MansionError;
    type Fut = futures_util::future::Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn checker(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let resp = match uncheck.as_str() {
            "false" => Ok(Predict::False),
            "unknown" => Ok(Predict::Unknown),
            "true" => Ok(Predict::True),
            _ => Err(UnknownPredictType.into()),
        };

        futures_util::future::ready(resp)
    }
}
