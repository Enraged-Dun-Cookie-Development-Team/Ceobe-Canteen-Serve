use std::marker::PhantomData;
use axum::body::Body;
use axum::response::{IntoResponse, Response};
use axum_resp_result::RespResult;
use http::Request;
use tap::Tap;
use tower_http::auth::AsyncAuthorizeRequest;
use tracing::Instrument;
use tracing_unwrap::OptionExt;
use persistence::help_crates::futures::future::BoxFuture;
use persistence::help_crates::futures::TryFutureExt;
use crate::author_verify::{ AuthorVerifier};
use crate::AuthorizedUser;

#[derive(Default,Clone)]
pub struct UserAuthorize<V>(V);

impl<V> UserAuthorize<V> {
    pub fn new(verify:V)->Self{
        Self(verify)
    }
}

impl<V> AsyncAuthorizeRequest<Body> for UserAuthorize<V>

where
    V: AuthorVerifier+ Send
{
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = BoxFuture<'static,Result<Request<Body>, Response>> ;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        let(mut parts,body) = request.into_parts();

        let payload =self.0.authorize(parts.clone());
        Box::pin(
            async move{

                match payload.await{
                    Ok(user)=>{
                        let request = Request::from_parts(parts,body);
                        Ok(request.tap_mut(|req|{
                            req.extensions_mut()
                                .insert(AuthorizedUser::<V::AuthorizedUser>(user))
                                .expect_none_or_log("Authorize Layer Exist")
                        }))
                    }
                    Err(err)=>{Err(RespResult::<(),_>::Err(err).into_response())}
                }
            }.instrument(tracing::info_span!("Authorization"))
        )
    }
}