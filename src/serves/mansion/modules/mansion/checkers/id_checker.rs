use futures_util::future::{ready, Ready};

use crate::{
    serves::mansion::error::{MansionError, UnknownId},
    utils::{
        data_checker::{DataChecker, OptionChecker},
    },
};

crate::check_obj! {
    {#[derive(Debug,serde::Deserialize)]}
    {#[derive(Debug,serde::Serialize, serde::Deserialize)]}
    pub struct MIdUncheck = MidChecker > Mid{
        #[serde(alias="idBefore",alias="mansionId")]
        pub id:IdChecker
    }
    err:MansionError
}

crate::check_obj! {
    {#[derive(Debug,serde::Deserialize)]}
    {#[derive(Debug,serde::Serialize)]}
    pub struct OpMIdUncheck = OpMidChecker > OptionMid{
        #[serde(alias="idBefore",alias="mansionId")]
       pub  id:OptionChecker<IdChecker>
    }
    err:MansionError
}

crate::quick_struct! {
    pub MansionId{
        main_id:u32
        minor_id:u8
    }
}

/// 饼学大厦号的检查器
/// ## Uncheck
/// [String](std::string::String)
/// ## Checked
/// (i32,i32)
#[derive(Debug)]
pub struct IdChecker;

impl DataChecker for IdChecker {
    type Args = ();
    type Checked = MansionId;
    type Err = MansionError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn checker(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let task = move || {
            let mut sp = uncheck.split(".");
            let f = sp.next().ok_or(UnknownId)?;
            let main_id = f.trim().parse::<u32>().map_err(|_| UnknownId)?;
            let n = sp.next().unwrap_or("0");
            let minor_id = n.trim().parse::<u8>().map_err(|_| UnknownId)?;
            // Next 还有东西，不行
            if let Some(_) = sp.next() {
                Err(UnknownId)?;
            }
            Ok(MansionId { main_id, minor_id })
        };

        ready(task())
    }
}

impl std::fmt::Display for MansionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.main_id, self.minor_id)
    }
}
