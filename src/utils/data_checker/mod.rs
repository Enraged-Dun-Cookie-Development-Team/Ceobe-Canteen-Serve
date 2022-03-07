pub mod collect_checkers;
pub mod codegen;
use std::marker::PhantomData;

use futures::Future;
use serde::Deserialize;

use super::req_pretreatment::Pretreatment;

pub mod no_check;

pub trait DataChecker {
    /// 未经过检查时的值
    type Unchecked;
    /// 检查时需要的外部信息
    type Args;
    /// 通过检查的值
    type Checked;
    /// 检查过程中出现的异常
    type Err;

    /// 检查过程可能为异步
    type Fut: Future<Output = Result<Self::Checked, Self::Err>>;

    /// 进行数据检查，可能为异步
    fn checker(args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut;

}

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

pub struct PretreatChecker<Pargs, Punchecked, C>(
    PhantomData<Pargs>,
    PhantomData<Punchecked>,
    PhantomData<C>,
);

impl<Pargs, Punchecked, C> Pretreatment for PretreatChecker<Pargs, Punchecked, C>
where
    C: DataChecker,
    Pargs: Pretreatment<Resp = C::Args>,
    Pargs::Err: Into<C::Err>,
    Punchecked: Pretreatment<Resp = C::Unchecked>,
    Punchecked::Err: Into<C::Err>,
    C::Checked: 'static,
{
    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    type Resp = C::Checked;

    type Err = C::Err;

    #[inline]
    fn call<'r>(
        req: &'r actix_web::HttpRequest,
        payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let args_fut = Pargs::call(req, payload);
        let uncheck_fut = Punchecked::call(req, payload);

        async move {
            let args = args_fut.await.map_err(Into::into)?;
            let uncheck = uncheck_fut.await.map_err(Into::into)?;

            let checked = C::checker(args, uncheck).await?;
            Ok(checked)
        }
    }
}
