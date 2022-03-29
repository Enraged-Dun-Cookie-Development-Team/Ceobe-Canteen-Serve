use core::fmt::Debug;
use serde::Deserialize;

use super::DataChecker;

pub struct CheckRequire<D: DataChecker>(D::Unchecked);

#[allow(dead_code)]
impl<D: DataChecker> CheckRequire<D>
where
    D::Checked: 'static,
{
    #[inline]
    pub fn new(_: D, unchecked: D::Unchecked) -> Self {
        CheckRequire(unchecked)
    }
    #[inline]
    pub async fn checking(self, args: D::Args) -> Result<D::Checked, D::Err> {
        D::checker(args, self.0).await
    }
    /// 直接获取未检查的数据将是不安全的
    #[inline]
    pub unsafe fn into_inner(self) -> D::Unchecked {
        self.0
    }
}

impl<D: DataChecker> Debug for CheckRequire<D> {
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
    Da: DataChecker,
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
