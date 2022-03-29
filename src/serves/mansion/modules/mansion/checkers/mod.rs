mod daily;
mod each_info;
mod id_checker;
mod mansion;

pub use daily::{Daily, DailyChecker, DailyUncheck};
pub use each_info::{EachInfoUncheck, Info, InfoChecker, Predict};
use futures::future::ok;
use futures_util::future::Ready;
pub use id_checker::{
    MIdUncheck, MansionId, Mid, MidChecker, OpMIdUncheck, OpMidChecker,
    OptionMid,
};
pub use mansion::{Mansion, MansionChecker, MansionUncheck};

use crate::{
    serves::mansion::error::MansionError,
    utils::{
        data_checker::{DataChecker, PretreatChecker},
        req_pretreatment::{
            prefabs::{Json, Null, Query},
            Pretreatment,
        },
    },
};

pub type MansionArgs = <MansionChecker as DataChecker>::Args;
pub struct MansionArgsLoader;
impl Pretreatment for MansionArgsLoader {
    type Err = MansionError;
    type Fut = Ready<Result<Self::Resp, Self::Err>>;
    type Resp = MansionArgs;

    fn call<'r>(
        _: &'r actix_web::HttpRequest, _: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let args = MansionArgs::default();
        ok(args)
    }
}

pub type MansionCheckerPretreat =
    PretreatChecker<MansionArgsLoader, Json<MansionUncheck>, MansionChecker>;

pub type MIdCheckerPretreat =
    PretreatChecker<Null, Query<MIdUncheck>, MidChecker>;
pub type OptionMidCheckerPretreat =
    PretreatChecker<Null, Query<OpMIdUncheck>, OpMidChecker>;
