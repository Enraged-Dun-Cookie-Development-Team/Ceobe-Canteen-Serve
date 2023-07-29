use futures::future::{ready, Ready};

use crate::Checker;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("版本号不存在：{version} ")]
pub struct VersionInvalidError {
    version: String,
}
pub struct VersionChecker;

impl Checker for VersionChecker {
    type Args = ();
    type Checked = String;
    type Err = VersionInvalidError;
    type Fut = Ready<Result<String, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let mut split_str = uncheck.split('.');

        ready(
            split_str
                .next()
                .zip(split_str.next())
                .zip(split_str.next())
                .filter(|_| split_str.next().is_none())
                .and_then(|((major_ver, minor_ver), security_ver)| {
                    Some((
                        major_ver.parse::<u32>().ok()?,
                        minor_ver.parse::<u32>().ok()?,
                        security_ver.parse::<u32>().ok()?,
                    ))
                })
                .map(|_| uncheck.clone())
                .ok_or(VersionInvalidError {
                    version: uncheck
                }),
        )
    }
}