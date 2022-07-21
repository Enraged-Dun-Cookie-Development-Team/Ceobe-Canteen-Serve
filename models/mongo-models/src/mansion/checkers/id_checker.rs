use checker::{check_obj, prefabs::option_checker::OptionChecker, Checker};
use futures_util::future::{ready, Ready};

use super::MansionDataCheckerError;
use crate::mansion::{checked::OptionMid, preludes::MansionId};

check_obj! {
    #[derive(Debug,serde::Deserialize)]
    pub struct OpMIdUncheck = OpMidChecker > OptionMid{
        #[serde(
            alias="idBefore",
            alias="id_before",
            alias="mansionId",
            alias="mansion_id"
        )]
        pub id: OptionChecker<IdChecker>
    }
    err:MansionDataCheckerError
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
    type Err = MansionDataCheckerError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = String;

    fn check(_args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        use MansionDataCheckerError::UnknownMansionIdFormat;
        let mut sp = uncheck.split(".");

        ready(
            sp.next()
                .zip(sp.next().or("0".into()))
                // 检查是否只有一个小数点
                .zip(if sp.next().is_none() { Some(()) } else { None })
                .map(|(core, _)| core)
                // 字符串转换成数字
                .and_then(|(main_str, minor_str)| {
                    Some((
                        main_str.trim().parse::<u32>().ok()?,
                        minor_str.trim().parse::<u8>().ok()?,
                    ))
                })
                .map(|(main_id, minor_id)| {
                    MansionId::builder()
                        .main_id(main_id)
                        .minor_id(minor_id)
                        .build()
                })
                .ok_or_else(|| UnknownMansionIdFormat(uncheck)),
        )
    }
}
