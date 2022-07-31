use checker::Checker;
use futures::future::{ready, Ready};

use super::CheckError;
use crate::addon_version::models::Version;

pub struct VersionChecker;

impl Checker for VersionChecker {
    type Args = ();
    type Checked = Version;
    type Err = CheckError;
    type Fut = Ready<Result<Version, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let mut spilt_str = uncheck.split(".");

        ready(
            spilt_str
                .next()
                .zip(spilt_str.next())
                .zip(spilt_str.next())
                .filter(|_| spilt_str.next().is_none())
                .and_then(|((major_ver, minor_ver), security_ver)| {
                    Some((
                        major_ver.parse::<u32>().ok()?,
                        minor_ver.parse::<u32>().ok()?,
                        security_ver.parse::<u32>().ok()?,
                    ))
                })
                .map(|(major_ver, minor_ver, security_ver)| {
                    Version(major_ver, minor_ver, security_ver)
                })
                .ok_or(CheckError::VersionFormat(uncheck)),
        )
    }
}

#[cfg(test)]
mod test {
    use checker::LiteChecker;

    use super::VersionChecker;

    #[test]
    fn test_good_version() {
        let uncheck = String::from("0.11.2");

        let resp = VersionChecker::lite_check(uncheck).into_inner().unwrap();

        println!("{:?}", resp)
    }
    
    #[test]
    #[should_panic]
    fn test_bad_version() {
        let uncheck = String::from("0.112.2rr.2");

        let resp = VersionChecker::lite_check(uncheck).into_inner().unwrap();

        println!("{:?}", resp)
    }


}
