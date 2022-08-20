use futures::future::{ready, Ready};
use url::Url;

use crate::Checker;

pub struct UrlChecker;

impl Checker for UrlChecker {
    type Args = ();
    type Checked = Url;
    type Err = url::ParseError;
    type Fut = Ready<Result<Url, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        ready(Url::parse(&uncheck))
    }
}
