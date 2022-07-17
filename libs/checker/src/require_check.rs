use core::fmt::Debug;

use serde::Deserialize;

use crate::{checker::LiteChecker, lite_args::LiteArgs, Checker};

pub struct CheckRequire<D: Checker>(D::Unchecked);

impl<D> CheckRequire<D>
where
    D: LiteChecker,
    <D as Checker>::Args: LiteArgs,
{
    pub async fn lite_checking(self) -> Result<D::Checked, D::Err> {
        D::lite_check(self.0).await
    }
}

#[allow(dead_code)]
impl<D: Checker> CheckRequire<D>
where
    D::Checked: 'static,
{
    #[inline]
    pub fn new(_: D, unchecked: D::Unchecked) -> Self {
        CheckRequire(unchecked)
    }

    #[inline]
    pub async fn checking(self, args: D::Args) -> Result<D::Checked, D::Err> {
        D::check(args, self.0).await
    }

    /// 直接获取未检查的数据将是不安全的
    #[inline]
    pub unsafe fn into_inner(self) -> D::Unchecked { self.0 }
}

impl<D: Checker> Debug for CheckRequire<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckRequire")
            .field("uncheck_type", &std::any::type_name::<D::Unchecked>())
            .field("checked_type", &std::any::type_name::<D::Checked>())
            .field("checker", &std::any::type_name::<D>())
            .finish()
    }
}
impl<'de, Da> Deserialize<'de> for CheckRequire<Da>
where
    Da: Checker,
    Da::Unchecked: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let res = Da::Unchecked::deserialize(deserializer)?;
        Ok(CheckRequire(res))
    }
}
