use checker::Checker;
use futures::future::{ready, Ready};

use super::CheckError;

pub struct DesktopVersionChecker;

impl Checker for DesktopVersionChecker {
    type Args = ();
    type Checked = String;
    type Err = CheckError;
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
                .ok_or(CheckError::VersionFormat(uncheck)),
        )
    }
}

#[cfg(test)]
mod test {
    use checker::LiteChecker;

    use super::DesktopVersionChecker;

    #[test]
    fn test_good_version() {
        let uncheck = String::from("0.11");

        let resp = DesktopVersionChecker::lite_check(uncheck)
            .into_inner()
            .unwrap();

        println!("{:?}", resp)
    }

    #[test]
    #[should_panic]
    fn test_bad_version() {
        let uncheck = String::from("0.112.2rr.2");

        let resp = DesktopVersionChecker::lite_check(uncheck)
            .into_inner()
            .unwrap();

        println!("{:?}", resp)
    }
}
