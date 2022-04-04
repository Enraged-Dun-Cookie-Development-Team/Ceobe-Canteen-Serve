mod check_require;
pub mod codegen;
pub mod collect_checkers;
mod load_from_args;
pub mod no_check;
mod ref_checker;

use std::{marker::PhantomData, any::type_name};

pub use check_require::*;
use futures::Future;
pub use ref_checker::RefChecker;
use time_usage::async_time_usage_with_name;

use super::req_pretreatment::Pretreatment;

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

pub struct PretreatChecker<Pargs, Punchecked, C>(
    PhantomData<Pargs>,
    PhantomData<Punchecked>,
    PhantomData<C>,
)
where
    C: DataChecker,
    Pargs: Pretreatment<Resp = C::Args>,
    Pargs::Err: Into<C::Err>,
    Punchecked: Pretreatment<Resp = C::Unchecked>,
    Punchecked::Err: Into<C::Err>,
    C::Checked: 'static;

impl<Pargs, Punchecked, C> Pretreatment
    for PretreatChecker<Pargs, Punchecked, C>
where
    C: DataChecker,
    Pargs: Pretreatment<Resp = C::Args>,
    Pargs::Err: Into<C::Err>,
    Punchecked: Pretreatment<Resp = C::Unchecked>,
    Punchecked::Err: Into<C::Err>,
    C::Checked: 'static,
{
    type Err = C::Err;
    type Resp = C::Checked;

    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    #[inline]
    fn proc(
        req: &actix_web::HttpRequest, payload: &mut actix_http::Payload,
    ) -> Self::Fut {
        let args_fut = Pargs::proc(req, payload);
        let uncheck_fut = Punchecked::proc(req, payload);

        async move {
            let args = async_time_usage_with_name(
                format!("加载检查器参数信息-{}", type_name::<C::Args>())
                    .as_str(),
                args_fut,
            )
            .await
            .map_err(Into::into)?;
            let uncheck = async_time_usage_with_name(
                format!("获取未检查数据-{}", type_name::<C::Unchecked>())
                    .as_str(),
                uncheck_fut,
            )
            .await
            .map_err(Into::into)?;

            let checked = async_time_usage_with_name(
                format!("执行检查-{}", type_name::<C>()).as_str(),
                C::checker(args, uncheck),
            )
            .await?;
            Ok(checked)
        }
    }
}

pub struct OptionChecker<C: DataChecker>(PhantomData<C>);

impl<C: DataChecker> DataChecker for OptionChecker<C> {
    type Args = C::Args;
    type Checked = Option<C::Checked>;
    type Err = C::Err;
    type Unchecked = Option<C::Unchecked>;

    type Fut = impl Future<Output = Result<Self::Checked, Self::Err>>;

    fn checker(args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        async move {
            match uncheck {
                Some(uc) => C::checker(args, uc).await.map(Some),
                None => Ok(Self::Checked::None),
            }
        }
    }
}
