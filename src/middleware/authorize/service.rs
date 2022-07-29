use std::{
    marker::{PhantomData, PhantomPinned},
    pin::Pin,
    task::Poll,
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
    config::get_authorize_information, decrypt_token,
};

#[derive(Clone)]
pub struct AuthorizeService<S, L> {
    inner: S,
    _pha: PhantomData<L>,
}

impl<S, L> Service<Request<Body>> for AuthorizeService<S, L>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
{
    type Error = S::Error;
    type Future = AuthorizeFut<S>;
    type Response = S::Response;

    fn poll_ready(
        &mut self, cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // 1 get req head
        let token = match get_authorize_information(&req) {
            Some(header) => header,
            None => {
                return AuthorizeFut {
                    state: AuthorizeFutState::Error(error_resp(
                        AuthorizeError::NoToken,
                    )),
                    __pha: PhantomPinned,
                }
            }
        };

        // parse token
        let user = match decrypt_token(token).map_err(AuthorizeError::from) {
            Ok(v) => v,
            Err(err) => {
                return AuthorizeFut {
                    state: AuthorizeFutState::Error(error_resp(err)),
                    __pha: PhantomPinned,
                }
            }
        };
        // query database
        let query_db_fut =
            Box::pin(UserSqlOperate::find_user_with_version_verify(
                user.id as i64,
                user.num_pwd_change,
                |user| user,
                AuthorizeError::TOkenInvalid,
            ));

        AuthorizeFut {
            state: AuthorizeFutState::QueryDatabase(
                query_db_fut,
                req,
                self.inner.clone(),
            ),
            __pha: PhantomPinned,
        }
    }
}

fn error_resp(err: AuthorizeError) -> Response {
    RespResult::<Nil, _>::err(err).into_response()
}

pub struct AuthorizeFut<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
{
    state: AuthorizeFutState<S>,
    __pha: PhantomPinned,
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
        #[pin]
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

impl<S> Future for AuthorizeFut<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Error: Send + 'static,
{
    type Output = Result<S::Response, S::Error>;

    fn poll(
        self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        let mut_this = unsafe { self.get_unchecked_mut() };
        let raw_state = &mut mut_this.state;
        let pin_state = unsafe { Pin::new_unchecked(raw_state) };
        let state = pin_state.project();

        let next = match state {
            // query database
            EnumProj::QueryDatabase(fut, req, inner) => {
                match fut.poll(cx) {
                    // query finish
                    Poll::Ready(resp) => {
                        match || -> Result<user::Model, AuthorizeError> {
                            Ok(resp.map_err(|err| {
                                match err {
                                    OperateError::UserNotExist => {
                                        AuthorizeError::TokenInfoNotFound
                                    }
                                    err => AuthorizeError::UserDbOperate(err),
                                }
                            })??)
                        }() {
                            // ok go ahead
                            Ok(model) => {
                                // set to req
                                req.extensions_mut()
                                    .insert(AuthorizeInfo(model));
                                // next fut
                                // take req
                                let req = std::mem::take(req);
                                let mut fut = inner.call(req);
                                let pin_fut =
                                    unsafe { Pin::new_unchecked(&mut fut) };
                                // poll next
                                match pin_fut.poll(cx) {
                                    Poll::Ready(v) => {
                                        return Poll::Ready(v);
                                    }
                                    Poll::Pending => {
                                        AuthorizeFutState::Inner(fut)
                                    }
                                }
                            }
                            // error return
                            Err(error) => {
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
                let err = std::mem::take(err);
                return Poll::Ready(Ok(err));
            }
        };
        // update state
        let raw_state = &mut mut_this.state;
        *raw_state = next;
        Poll::Pending
    }
}
