use std::{
    fmt::{Debug, Display},
    io::{Cursor, Write},
};

use futures::future::{ready, Ready};

use super::CheckError;

#[derive(PartialEq, Eq)]
pub struct Bv([u8; 12]);

impl Debug for Bv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8_lossy(&self.0).to_string();
        f.debug_tuple("Bv").field(&s).finish()
    }
}

impl Display for Bv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8_lossy(&self.0);
        write!(f, "{}", s)
    }
}

pub struct BvChecker;

impl checker::Checker for BvChecker {
    type Args = ();
    type Checked = Bv;
    type Err = CheckError;
    type Fut = Ready<Result<Bv, Self::Err>>;
    type Unchecked = String;

    fn check(_: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        let task = || {
            if uncheck.len() == 12 {
                let chars = uncheck.chars().take(12);

                if chars
                    .clone()
                    .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
                    && uncheck.to_lowercase().starts_with("bv")
                {
                    let mut bv = Cursor::new([0u8; 12]);
                    bv.write_all(
                        &chars.map(|c| c as u8).collect::<Vec<_>>()[0..12],
                    )
                    .ok();

                    return Ok(Bv(bv.into_inner()));
                }
            }

            Err(CheckError::WrongBv(uncheck))
        };

        ready(task())
    }
}

#[cfg(test)]
mod test {
    use checker::CheckRequire;

    use crate::ceobe_operation::video::checkers::CheckError;

    use super::BvChecker;
    #[tokio::test]
    async fn test_bv_succeed() {
        let uncheck = CheckRequire::new(BvChecker, "BV1ZB4y1Y7Hm".into());

        let checked = uncheck.lite_checking().await.unwrap();

        assert_eq!(checked.to_string(), "BV1ZB4y1Y7Hm")
    }
    #[tokio::test]
    async fn test_bv_failure() {
        let uncheck = CheckRequire::new(BvChecker, "Av170001".into());

        let checked = uncheck.lite_checking().await;

        assert_eq!(
            Err(CheckError::WrongBv("Av170001".into())),
            checked
        );
    }
}
