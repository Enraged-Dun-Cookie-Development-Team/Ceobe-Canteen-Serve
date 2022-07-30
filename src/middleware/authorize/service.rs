use std::{
    marker::{PhantomData, PhantomPinned},
    mem::take,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    body::Body,
    response::{IntoResponse, Response},
};
use futures::{future::BoxFuture, Future};
use http::Request;
use orm_migrate::sql_models::admin_user::{
    models::user,
    operate::{OperateError, UserSqlOperate},
};
use pin_project::pin_project;
use resp_result::{Nil, RespResult};
use tower::Service;

use super::{error::AuthorizeError, AuthorizeInfo};
use crate::utils::user_authorize::{
    auth_level::{AuthLevelVerify, UnacceptableAuthorizationLevelError},
    config::get_authorize_information,
    decrypt_token,
};

pub struct AuthorizeService<S, L> {
    pub(super) inner: S,
    pub(super) _pha: PhantomData<L>,
}

impl<S: Clone, L> Clone for AuthorizeService<S, L> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _pha: PhantomData,
        }
    }
}

impl<S, L> Service<Request<Body>> for AuthorizeService<S, L>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
    L: AuthLevelVerify,
{
    type Error = S::Error;
    type Future = AuthorizeFut<S, L>;
    type Response = S::Response;

    fn poll_ready(
        &mut self, cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // 1 get req head
        log::info!("鉴权-> 从请求头中加载用户登录信息");
        let token = match get_authorize_information(&req) {
            Some(header) => header,
            None => {
                log::error!("获取用户登录信息失败");
                return AuthorizeFutState::new_error(AuthorizeError::NoToken)
                    .into();
            }
        };

        log::info!("鉴权-> 解析用户登录信息");
        // parse token
        let user = match decrypt_token(token).map_err(AuthorizeError::from) {
            Ok(v) => v,
            Err(err) => {
                log::error!("解析用户信息失败");
                return AuthorizeFutState::new_error(err).into();
            }
        };

        log::info!("鉴权-> 查询并确认用户信息");
        // query database
        let query_db_fut =
            Box::pin(UserSqlOperate::find_user_with_version_verify(
                user.id as i64,
                user.num_pwd_change,
                |user| user,
                AuthorizeError::TOkenInvalid,
            ));

        AuthorizeFutState::new_db_query(query_db_fut, req, self.inner.clone())
            .into()
    }
}

fn error_resp(err: AuthorizeError) -> Response {
    RespResult::<Nil, _>::err(err).into_response()
}

pub struct AuthorizeFut<S, L>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
    L: AuthLevelVerify,
{
    state: AuthorizeFutState<S>,
    _pha_auth_level: PhantomData<L>,
    __pha: PhantomPinned,
}

impl<S, L> From<AuthorizeFutState<S>> for AuthorizeFut<S, L>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
    L: AuthLevelVerify,
{
    fn from(state: AuthorizeFutState<S>) -> Self {
        Self {
            state,
            _pha_auth_level: PhantomData,
            __pha: PhantomPinned,
        }
    }
}

#[pin_project(project = EnumProj)]
pub enum AuthorizeFutState<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
{
    // step 1 get request header(sync)
    // parse token
    // step 2 query database (async)
    QueryDatabase(
        // query db fut
        BoxFuture<
            'static,
            Result<Result<user::Model, AuthorizeError>, OperateError>,
        >,
        // for next step
        Request<Body>,
        S,
    ),
    // set user info in to request
    // inner fut
    Inner(#[pin] S::Future),

    // something wrong
    Error(S::Response),
}

impl<S> AuthorizeFutState<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
{
    fn new_error(err: impl Into<AuthorizeError>) -> Self {
        Self::Error(RespResult::<Nil, _>::err(err.into()).into_response())
    }

    fn new_inner(fut: S::Future) -> Self { Self::Inner(fut) }

    fn new_db_query(
        box_future: BoxFuture<
            'static,
            Result<Result<user::Model, AuthorizeError>, OperateError>,
        >,
        req: Request<Body>, service: S,
    ) -> Self {
        Self::QueryDatabase(box_future, req, service)
    }
}

impl<S, L> Future for AuthorizeFut<S, L>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
    L: AuthLevelVerify,
{
    type Output = Result<S::Response, S::Error>;

    fn poll(
        self: Pin<&mut Self>, cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        // get mut of self , make sure self never move
        let mut state = unsafe {
            Pin::new_unchecked(&mut self.get_unchecked_mut().state)
        };
        // pinned the state then project in match
        let next = match state.as_mut().project() {
            // query database
            EnumProj::QueryDatabase(fut, req, inner) => {
                match fut.as_mut().poll(cx) {
                    // query finish
                    Poll::Ready(resp) => {
                        match || -> Result<user::Model, AuthorizeError> {
                            resp.map_err(|err| {
                                match err {
                                    OperateError::UserNotExist => {
                                        AuthorizeError::TokenInfoNotFound
                                    }
                                    err => AuthorizeError::UserDbOperate(err),
                                }
                            })?
                        }() {
                            // ok go ahead
                            Ok(model) => {
                                log::debug!(
                                    "鉴权-> 用户信息查询完成, \
                                     检查权限等级是否匹配"
                                );
                                // verify user authorize level
                                if !L::verify(&model.auth) {
                                    return Poll::Ready(Ok(error_resp(UnacceptableAuthorizationLevelError::new(L::auth_name()).into())));
                                }

                                log::debug!(
                                    "鉴权-> 鉴权通过! \
                                     将用户信息添加到Request"
                                );
                                // set to req
                                req.extensions_mut()
                                    .insert(AuthorizeInfo(model));
                                // next fut
                                // take req
                                log::debug!("鉴权-> 执行内部service");
                                let req = take(req);
                                let mut fut = inner.call(req);
                                // poll next
                                match unsafe { Pin::new_unchecked(&mut fut) }
                                    .poll(cx)
                                {
                                    Poll::Ready(v) => {
                                        return Poll::Ready(v);
                                    }
                                    Poll::Pending => {
                                        log::debug!(
                                            "鉴权-> 内部Service未完成, \
                                             状态切换"
                                        );
                                    }
                                }
                                AuthorizeFutState::new_inner(fut)
                            }
                            // error return
                            Err(error) => {
                                log::error!(
                                    "鉴权-> 用户信息查询时出现异常 {}",
                                    error
                                );
                                let resp = error_resp(error);
                                return Poll::Ready(Ok(resp));
                            }
                        }
                    }
                    // not finish yet waiting
                    Poll::Pending => return Poll::Pending,
                }
            }
            EnumProj::Inner(fut) => return fut.poll(cx),
            EnumProj::Error(err) => {
                log::error!("鉴权-> 出现异常");
                let err = take(err);
                return Poll::Ready(Ok(err));
            }
        };
        state.set(next);
        Poll::Pending
    }
}
