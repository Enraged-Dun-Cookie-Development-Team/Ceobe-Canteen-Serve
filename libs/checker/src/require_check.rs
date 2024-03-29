use core::fmt::Debug;

use serde::Deserialize;

use crate::{checker::LiteChecker, lite_args::LiteArgs, CheckFut, Checker};

pub struct CheckRequire<D: Checker>(D::Unchecked);

impl<D> CheckRequire<D>
where
    D: Checker,
    D::Checked: 'static,
{
    #[inline]
    #[allow(dead_code)]
    pub fn into_check_fut(self, args: D::Args) -> CheckFut<D> {
        CheckFut::Fut(self.checking(args))
    }
}

impl<D> CheckRequire<D>
where
    D: LiteChecker,
    <D as Checker>::Args: LiteArgs,
{
    #[inline]
    pub fn lite_checking(self) -> D::Fut { D::lite_check(self.0) }
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
    pub fn new_with_no_checker(unchecked: D::Unchecked) -> Self {
        CheckRequire(unchecked)
    }

    #[inline]
    pub fn checking(self, args: D::Args) -> D::Fut { D::check(args, self.0) }
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
/// 将当前类型转换为指定的 [`Checker`](Checker) 检查的 [CheckRequire]
pub trait ToCheckRequire: Sized {
    /// 将Self转换为对应的 [CheckRequire].
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::prefabs::no_check::NoCheck;
    /// use crate::require_check::{CheckRequire, ToCheckRequire};
    ///
    /// let _cr : CheckRequire<NoCheck<i32>> = 11i32.require_check();
    /// ```
    #[inline]
    fn require_check<C: Checker<Unchecked = Self>>(self) -> CheckRequire<C> {
        CheckRequire(self)
    }
}

impl<T: Sized> ToCheckRequire for T {}
