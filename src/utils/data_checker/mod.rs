use async_trait::async_trait;
use axum::{body::Body, extract::RequestParts};
use axum_prehandle::PreHandler;
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

use std::{any::type_name, marker::PhantomData};

pub use check_require::*;
pub use ref_checker::RefChecker;
use time_usage::async_time_usage_with_name;

pub struct PreLiteChecker<Punchecked, C, E>(PhantomData<(Punchecked, C, E)>)
where
    C: DataChecker,
    <C as DataChecker>::Args: LiteArgs,
    Punchecked: PreHandler<Body, Output = C::Unchecked>,
    E: 'static + From<C::Err> + From<Punchecked::Rejection>,
    C::Checked: 'static;

#[async_trait]
impl<Punchecked, C, E> PreHandler<Body> for PreLiteChecker<Punchecked, C, E>
where
    C: DataChecker,
    C::Unchecked: Send,
    C::Fut: Send,
    <C as DataChecker>::Args: LiteArgs + Send,
    Punchecked: PreHandler<Body, Output = C::Unchecked>,
    E: 'static + From<C::Err> + From<Punchecked::Rejection>,
    C::Checked: 'static,
{
    type Output = C::Checked;
    type Rejection = E;

    async fn handling(
        request: &mut RequestParts<Body>,
    ) -> Result<Self::Output, Self::Rejection> {
        let args = <<C as DataChecker>::Args as LiteArgs>::get_arg();
        let uncheck = async_time_usage_with_name(
            format!("获取未检查数据-{}", type_name::<C::Unchecked>())
                .as_str(),
            Punchecked::handling(request),
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
