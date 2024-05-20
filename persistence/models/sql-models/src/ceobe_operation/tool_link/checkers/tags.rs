use checker::{prefabs::{post_checker::PostChecker, str_len_checker::StrMaxCharLenChecker}, Checker};
use futures::future::{ready, Ready};

use super::CheckError;



pub struct TagSerializeCheck;

impl Checker for TagSerializeCheck {
    type Args = ();
    type Checked = String;
    type Err = CheckError;
    type Fut = Ready<Result<String, Self::Err>>;
    type Unchecked = Vec<String>;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let task = || {
            Ok(serde_json::to_string(&uncheck)?)
        };

        ready(task())
    }
}

pub type TagsChecker = PostChecker<
    TagSerializeCheck,
    StrMaxCharLenChecker<String, 128>,
    CheckError,
>;
