use std::marker::PhantomData;

use futures::Future;

use super::{error::UnacceptableAuthorizationLevelError, AuthLevelVerify};
use crate::utils::{
    req_pretreatment::Pretreatment,
    user_authorize::{
        error::AuthError, AuthInfo, TokenAuth, VerifiedAuthInfo,
    },
};

pub struct AuthLevel<L: AuthLevelVerify>(PhantomData<L>);

impl<L: AuthLevelVerify> Pretreatment for AuthLevel<L> {
    type Err = AuthError;
    type Resp = VerifiedAuthInfo;

    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    fn call<'r>(
        req: &'r actix_web::HttpRequest, payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let task = TokenAuth::call(req, payload);

        async move {
            let info = task.await?;
            let AuthInfo { id, auth, username } = info;

            if L::verify(&auth) {
                Ok(VerifiedAuthInfo { id, username })
            }
            else {
                Err(UnacceptableAuthorizationLevelError::new(L::auth_name())
                    .into())
            }
        }
    }
}
