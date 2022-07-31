use checker::{prefabs::url_checker::UrlChecker, Checker, LiteChecker};
use futures::future::{ready, Ready};
use range_limit::limits::max_limit::MaxRangeLimit;

use super::{CheckError, SpareLink};

pub struct SpareLinkChecker;

impl Checker for SpareLinkChecker {
    type Args = ();
    type Checked = SpareLink;
    type Err = CheckError;
    type Fut = Ready<Result<Self::Checked, Self::Err>>;
    type Unchecked = (String, String);

    fn check(_: Self::Args, (link, label): Self::Unchecked) -> Self::Fut {
        ready(
            UrlChecker::lite_check(link)
                .into_inner()
                .map_err(Into::into)
                .and_then(|url| {
                    MaxRangeLimit::<String, 64>::lite_check(label)
                        .into_inner()
                        .map_err(Into::into)
                        .map(|label| (url, label))
                })
                .map(|(url, label)| SpareLink(url, label)),
        )
    }
}
