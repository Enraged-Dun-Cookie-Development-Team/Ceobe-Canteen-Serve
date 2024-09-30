use std::{
    fmt::{Debug, Display},
    io::{Cursor, Write},
};

use futures::future::{ready, Ready};
use once_cell::sync::Lazy;
use regex::Regex;

use super::CheckError;

static BV_PATTEN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^BV[a-zA-Z0-9]{10}$"#).expect("正则表达式格式错误")
});

#[derive(PartialEq, Eq)]
pub struct Bv([u8; 12]);

impl Bv {
    /// # Safety
    /// 使用的 [Bv] 来自 [BvChecker] , 将保证这个过程是安全的
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl AsRef<str> for Bv {
    fn as_ref(&self) -> &str { self.as_str() }
}

impl Debug for Bv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bv").field(&self.as_str()).finish()
    }
}

impl Display for Bv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
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
            if BV_PATTEN.is_match(&uncheck) {
                let chars = uncheck.chars().take(12);
                let mut bv = Cursor::new([0u8; 12]);
                bv.write_all(
                    &chars.map(|c| c as u8).collect::<Vec<_>>()[0..12],
                )
                .ok();

                Ok(Bv(bv.into_inner()))
            }
            else {
                Err(CheckError::WrongBv(uncheck))
            }
        };

        ready(task())
    }
}

#[cfg(test)]
mod test {
    use checker::CheckRequire;

    use super::BvChecker;
    use crate::ceobe_operation::video::checkers::CheckError;

    #[tokio::test]
    async fn test_bv_succeed() {
        let uncheck = CheckRequire::new(BvChecker, "BV1ZB4y1Y7Hm".into());

        let checked = uncheck.lite_checking().await.unwrap();

        println!("{}", checked);
        assert_eq!(checked.to_string(), "BV1ZB4y1Y7Hm")
    }
    #[tokio::test]
    async fn test_bv_failure() {
        let uncheck = CheckRequire::new(BvChecker, "Av170001".into());

        let checked = uncheck.lite_checking().await;

        println!(
            "{}",
            match &checked {
                Ok(_) => unreachable!(),
                Err(err) => err,
            }
        );
        assert_eq!(Err(CheckError::WrongBv("Av170001".into())), checked);
    }
}
