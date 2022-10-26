use std::marker::PhantomData;

use async_trait::async_trait;
use axum::{body::Body, extract::RequestParts};
use axum_prehandle::PreHandler;

use crate::{Checker as DataChecker, LiteArgs};
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
        let uncheck = Punchecked::handling(request).await?;

        let checked = C::check(args, uncheck).await?;
        Ok(checked)
    }
}
