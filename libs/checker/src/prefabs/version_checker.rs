use std::{fmt::Display, marker::PhantomData};

use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

use crate::Checker;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub security: u32,
}

impl Version {
    pub fn to_version_str(self) -> String { self.to_string() }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.security)
    }
}

pub trait FromVersion {
    fn from_version(version: Version) -> Self;
}

impl FromVersion for Version {
    fn from_version(version: Version) -> Self { version }
}

impl FromVersion for String {
    fn from_version(version: Version) -> Self { version.to_version_str() }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("版本号格式错误：{version} ")]
pub struct VersionInvalidError {
    version: String,
}
pub struct VersionChecker<T>(PhantomData<T>);

impl<T: FromVersion> Checker for VersionChecker<T> {
    type Args = ();
    type Checked = T;
    type Err = VersionInvalidError;
    type Fut = Ready<Result<T, Self::Err>>;
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
                .map(|(major_ver, minor_ver, security_ver)| {
                    FromVersion::from_version(Version {
                        major: major_ver,
                        minor: minor_ver,
                        security: security_ver,
                    })
                })
                .ok_or(VersionInvalidError { version: uncheck }),
        )
    }
}
