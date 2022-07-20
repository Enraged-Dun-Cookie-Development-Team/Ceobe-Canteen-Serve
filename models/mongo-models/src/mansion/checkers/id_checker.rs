use checker::{Checker,check_obj, prefabs::option_checker::OptionChecker};
use futures_util::future::{ready, Ready};

use crate::mansion::{preludes::MansionId, MansionDataError, check::OptionMid};


check_obj! {
    #[derive(Debug,serde::Deserialize)]
    pub struct OpMIdUncheck = OpMidChecker > OptionMid{
        #[serde(alias="idBefore",alias="id_before",alias="mansionId",alias="mansion_id")]
       pub  id:OptionChecker<IdChecker>
    }
    err:MansionDataError
}

/// 饼学大厦号的检查器
/// ## Uncheck
/// [String](std::string::String)
/// ## Checked
/// (i32,i32)
#[derive(Debug)]
pub struct IdChecker;

impl Checker for IdChecker {
    type Args = ();
    type Checked = MansionId;
    type Err = MansionDataError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let task = move || {
            let mut sp = uncheck.split('.');
            let f = sp.next().ok_or(MansionDataError::UnknownId)?;
            let main_id = f.trim().parse::<u32>().map_err(|_| MansionDataError::UnknownId)?;
            let n = sp.next().unwrap_or("0");
            let minor_id = n.trim().parse::<u8>().map_err(|_| MansionDataError::UnknownId)?;
            // Next 还有东西，不行
            if sp.next().is_some() {
                Err(MansionDataError::UnknownId)?;
            }
            Ok(MansionId { main_id, minor_id })
        };

        ready(task())
    }
}
