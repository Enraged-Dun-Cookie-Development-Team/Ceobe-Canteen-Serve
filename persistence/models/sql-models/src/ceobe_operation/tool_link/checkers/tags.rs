use checker::Checker;
use futures::future::{ready, Ready};

use super::CheckError;

pub struct TagSerializeChecker;

impl Checker for TagSerializeChecker {
    type Args = ();
    type Checked = String;
    type Err = CheckError;
    type Fut = Ready<Result<String, Self::Err>>;
    type Unchecked = Vec<String>;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ready(serde_json::to_string(&uncheck).map_err(Into::into))
    }
}
