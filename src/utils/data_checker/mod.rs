mod load_from_args;

use checker::LiteArgs;
pub use checker::{
    prefabs::option_checker::OptionChecker, Checker as DataChecker,
};

pub mod no_check {
    pub use checker::prefabs::no_check::NoCheck;
}

mod ref_checker {
    pub use checker::RefChecker;
}

pub mod collect_checkers {
    pub use checker::prefabs::collect_checkers::*;
}

pub mod codegen {
    pub use checker::check_obj;
}

mod check_require {
    pub use checker::CheckRequire;
}

use std::{any::type_name, marker::PhantomData, future::Future};

pub use check_require::*;
pub use ref_checker::RefChecker;
use time_usage::async_time_usage_with_name;

use super::req_pretreatment::Pretreatment;

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
                C::check(args, uncheck),
            )
            .await?;
            Ok(checked)
        }
    }
}

pub struct PreLiteChecker<Punchecked, C, E>(PhantomData<(Punchecked, C, E)>)
where
    C: DataChecker,
    <C as DataChecker>::Args: LiteArgs,
    Punchecked: Pretreatment<Resp = C::Unchecked>,
    E: 'static + From<C::Err> + From<Punchecked::Err>,
    C::Checked: 'static;

impl<Punchecked, C, E> Pretreatment for PreLiteChecker<Punchecked, C, E>
where
    C: DataChecker,
    <C as DataChecker>::Args: LiteArgs,
    Punchecked: Pretreatment<Resp = C::Unchecked>,
    E: 'static + From<C::Err> + From<Punchecked::Err>,
    C::Checked: 'static,
{
    type Err = E;
    type Resp = C::Checked;

    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    fn proc(
        req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload,
    ) -> Self::Fut {
        let args = <<C as DataChecker>::Args as LiteArgs>::get_arg();
        let uncheck_fut = Punchecked::proc(req, payload);

        async move {
            let uncheck = async_time_usage_with_name(
                format!("获取未检查数据-{}", type_name::<C::Unchecked>())
                    .as_str(),
                uncheck_fut,
            )
            .await?;

            let checked = async_time_usage_with_name(
                format!("执行检查-{}", type_name::<C>()).as_str(),
                C::check(args, uncheck),
            )
            .await?;
            Ok(checked)
        }
    }
}
