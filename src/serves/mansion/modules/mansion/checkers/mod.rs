mod daily;
mod each_info;
mod mansion;

pub use daily::{Daily, DailyChecker, DailyUncheck};
pub use each_info::{EachInfoUncheck, Info, InfoChecker, Predict};
pub use mansion::{Mansion, MansionChecker, MansionUncheck};

use futures::future::ok;
use crate::{
    serves::mansion::error::MansionError,
    utils::{
        data_checker::{DataChecker, PretreatChecker},
        req_pretreatment::{prefabs::Json, Pretreatment},
    },
};

pub type MansionArgs = <MansionChecker as DataChecker>::Args;
pub struct MansionArgsLoader;
impl Pretreatment for MansionArgsLoader {
    type Fut = futures_util::future::Ready<Result<Self::Resp, Self::Err>>;

    type Resp = MansionArgs;

    type Err = MansionError;

    fn call<'r>(_: &'r actix_web::HttpRequest, _: &'r mut actix_http::Payload) -> Self::Fut {
        let args = MansionArgs::default();
        ok(args)
    }
}

pub type MansionCheckerPretreat =
    PretreatChecker<MansionArgsLoader, Json<MansionUncheck>, MansionChecker>;
